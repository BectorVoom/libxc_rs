//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1367/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1367(t10469: f64, t3502: f64, t478: f64, t11702: f64, t7339: f64, t24684: f64, t27634: f64, t1210: f64, t24654: f64, t24721: f64, t11168: f64, t11809: f64, t11855: f64, t11863: f64, t2121: f64, t24664: f64, t24670: f64, t24736: f64, t27636: f64, t27638: f64, t27644: f64, t3448: f64, t3493: f64, t3503: f64, t3531: f64, t7345: f64) -> (f64, f64) {
    let t86214 = t10469 * t3502 * t478;
    let t86228 = t7339 * t11702;
    let t86234 = t27634 * t24684;
    let t86248 = t24721 * t1210 * t24654;
    let t86253 = -t24736 * t3531 / 384.0_f64 - t7345 * t11809 / 384.0_f64 + t86228 / 768.0_f64 + t7339 * t11855 / 1536.0_f64 - t7345 * t11863 / 384.0_f64 - 0.60559134141210586284e-3_f64 * t86234 * t24664 + 0.30279567070605293142e-3_f64 * t86234 * t24670 + 0.60559134141210586284e-3_f64 * t27636 * t3503 * t3493 * t27638 - 0.30279567070605293142e-3_f64 * t27636 * t1210 * t3493 * t27644 + 0.30279567070605293142e-3_f64 * t86248 - t2121 * t3448 * t11168 / 48.0_f64;
    (t86214, t86253)
}
