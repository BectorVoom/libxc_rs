//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta443 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1750;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1751;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta443(t22833: f64, t3809: f64, t2002: f64, t3773: f64, t559: f64, t1878: f64, t557: f64, t3766: f64, t556: f64, t598: f64, t213: f64, t1998: f64, t236: f64, t3734: f64, t3872: f64, t6952: f64, t281: f64, t6931: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22834, t22836, t22837, t22839, t22840, t22842, t22843, t22844, t22845, t22847) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1750(t22833, t3809, t2002, t3773, t559, t1878, t557, t3766, t556, t598, t213, t1998, t236, t3734);
        let (t22848, t22850, t22852) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1751(t22845, t22847, t3872, t6952, t281, t6931);
    (t22834, t22836, t22837, t22839, t22840, t22842, t22843, t22844, t22847, t22848, t22850, t22852)
}
