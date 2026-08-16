//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta418 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1591;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1592;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta418<F: Float>(t22833: F, t3809: F, t2002: F, t3773: F, t559: F, t1878: F, t557: F, t3766: F, t556: F, t598: F, t213: F, t1998: F, t236: F, t3734: F, t3872: F, t6952: F, t281: F, t6931: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t22834, t22836, t22837, t22839, t22840, t22842, t22843, t22844, t22845, t22847) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1591::<F>(t22833, t3809, t2002, t3773, t559, t1878, t557, t3766, t556, t598, t213, t1998, t236, t3734);
        let (t22848, t22850, t22852) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1592::<F>(t22845, t22847, t3872, t6952, t281, t6931);
    (t22834, t22836, t22837, t22839, t22840, t22842, t22843, t22844, t22847, t22848, t22850, t22852)
}
