//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 805/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk805(t39644: f64, t5345: f64, t5348: f64, t1692: f64, t2519: f64, t12568: f64, t716: f64, t871: f64, t9678: f64, t2524: f64, t3113: f64, t2554: f64, t7064: f64, t9642: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40630 = t5345 * t39644 * t5348;
    let t40632 = t1692 * t2519;
    let t40634 = t12568 * t716;
    let t40640 = t9678 * t871;
    let t40641 = t2524 * t3113;
    let t40693 = t7064 * t9642 * t2554;
    (t40630, t40632, t40634, t40640, t40641, t40693)
}
