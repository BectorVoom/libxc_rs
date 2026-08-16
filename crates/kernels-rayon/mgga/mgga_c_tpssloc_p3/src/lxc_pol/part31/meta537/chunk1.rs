//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1753/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1753(t15: f64, t2229: f64, t1361: f64, t192: f64, t1995: f64, t22690: f64, t2230: f64, t22843: f64, t213: f64, t22842: f64, t531: f64, t598: f64) -> (f64, f64, f64, f64, f64) {
    let t80881 = 1.0_f64 / t2229 / t15;
    let t80885 = t80881 * t1995 * t192 * t22690 * t1361;
    let t80887 = t2230 * t22843;
    let t80888 = t80887 * t213;
    let t80893 = t598 / t22842 / t531;
    (t80881, t80885, t80887, t80888, t80893)
}
