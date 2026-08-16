//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 259/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk259(t1036: f64, t370: f64, t368: f64, t372: f64, t364: f64, t354: f64) -> (f64, f64, f64) {
    let t1038 = t370 * t1036 / 4608.0_f64;
    let t1039 = t368 * t372;
    let t1040 = t364 * t1039;
    let t1041 = t354 * t1040;
    (t1038, t1040, t1041)
}
