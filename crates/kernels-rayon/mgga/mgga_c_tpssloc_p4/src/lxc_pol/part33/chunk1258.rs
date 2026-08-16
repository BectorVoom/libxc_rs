//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1258/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1258(t7736: f64, t80854: f64, t81064: f64, t22642: f64, t22690: f64, t26395: f64, t22863: f64, t7737: f64, t3787: f64, t7722: f64, t26426: f64, t81046: f64) -> (f64, f64, f64, f64, f64) {
    let t90980 = t81064 * t80854 * t7736;
    let t90993 = t22642 * t22690 * t26395;
    let t91000 = t22863 * t7737;
    let t91029 = t3787 * t7722;
    let t91078 = t81046 * t26426;
    (t90980, t90993, t91000, t91029, t91078)
}
