//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1367/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1367<F: Float>(t10469: F, t3502: F, t478: F, t11702: F, t7339: F, t24684: F, t27634: F, t1210: F, t24654: F, t24721: F, t11168: F, t11809: F, t11855: F, t11863: F, t2121: F, t24664: F, t24670: F, t24736: F, t27636: F, t27638: F, t27644: F, t3448: F, t3493: F, t3503: F, t3531: F, t7345: F) -> (F, F) {
    let t86214 = t10469 * t3502 * t478;
    let t86228 = t7339 * t11702;
    let t86234 = t27634 * t24684;
    let t86248 = t24721 * t1210 * t24654;
    let t86253 = -t24736 * t3531 / F::cast_from(384.0_f64) - t7345 * t11809 / F::cast_from(384.0_f64) + t86228 / F::cast_from(768.0_f64) + t7339 * t11855 / F::cast_from(1536.0_f64) - t7345 * t11863 / F::cast_from(384.0_f64) - F::cast_from(0.60559134141210586284e-3_f64) * t86234 * t24664 + F::cast_from(0.30279567070605293142e-3_f64) * t86234 * t24670 + F::cast_from(0.60559134141210586284e-3_f64) * t27636 * t3503 * t3493 * t27638 - F::cast_from(0.30279567070605293142e-3_f64) * t27636 * t1210 * t3493 * t27644 + F::cast_from(0.30279567070605293142e-3_f64) * t86248 - t2121 * t3448 * t11168 / F::cast_from(48.0_f64);
    (t86214, t86253)
}
