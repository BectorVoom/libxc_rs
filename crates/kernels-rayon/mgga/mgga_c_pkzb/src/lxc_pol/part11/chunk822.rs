//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 822/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk822(t7012: f64, t114: f64, t3380: f64, t557: f64, t5078: f64, t5080: f64, t126: f64, t8748: f64, t83: f64, t545: f64, t3501: f64, t5165: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8769 = 16.0_f64 * t7012;
    let t8770 = t3380 * t114;
    let t8771 = t8770 * t557;
    let t8772 = 0.5848223622634646207e0_f64 * t8771;
    let t8773 = 8.0_f64 * t5078;
    let t8774 = 8.0_f64 * t5080;
    let t8775 = t8748 * t126;
    let t8776 = t83 * t8775;
    let t8777 = t3380 * t545;
    let t8778 = t83 * t8777;
    let t8779 = t3501 * t5165;
    (t8769, t8770, t8771, t8772, t8773, t8774, t8775, t8776, t8777, t8778, t8779)
}
