//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1129/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1129(t7432: f64, t84241: f64, t45844: f64, t7025: f64, t12571: f64, t23966: f64, t23993: f64, t7428: f64, t1860: f64, t23992: f64, t7445: f64, t26012: f64, t7031: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t91922 = t84241 * t7432;
    let t91954 = t45844 * t7025;
    let t91957 = t12571 * t23966;
    let t91996 = t7428 * t23993;
    let t92003 = t1860 * t23992 * t7445;
    let t92047 = t7031 * t26012;
    (t91922, t91954, t91957, t91996, t92003, t92047)
}
