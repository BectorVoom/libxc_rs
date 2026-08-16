//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta168 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk826;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk827;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta168(t28: f64, t1081: f64, t3231: f64, t3672: f64, t517: f64, t157: f64, t3671: f64, zeta_threshold: f64, t182: f64, t118: f64, t521: f64) -> (f64, f64, f64, f64) {
        let (t3673, t3681) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk826(t28, t1081, t3231, t3672, t517, t157, t3671, zeta_threshold);
        let (t3683, t3684) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk827(t182, t3681, t118, t521);
    (t3673, t3681, t3683, t3684)
}
