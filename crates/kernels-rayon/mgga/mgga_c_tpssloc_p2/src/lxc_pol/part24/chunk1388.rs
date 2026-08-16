//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1388/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1388(t10383: f64, t1926: f64, t10394: f64, t10398: f64, t10405: f64, t10419: f64, t10482: f64, t10860: f64, t1937: f64, t23413: f64, t23414: f64, t23419: f64, t23453: f64, t23495: f64, t3073: f64, t6722: f64, t6729: f64, t6730: f64, t6735: f64, t6747: f64, t6755: f64, t82981: f64, t82987: f64, t82989: f64, t82990: f64, t82996: f64, t83004: f64, t83008: f64, t83016: f64, t83025: f64) -> f64 {
    let t83028 = 5.0_f64 / 1296.0_f64 * t1926 * t10383;
    let t83029 = 0.30279567070605293142e-3_f64 * t82981 * t6747 + 0.60559134141210586284e-3_f64 * t82987 * t82989 * t82990 * t10482 + 0.30279567070605293142e-3_f64 * t82996 + t6755 * t10860 / 1536.0_f64 - 0.30279567070605293142e-3_f64 * t23414 * t6735 - 0.30279567070605293142e-3_f64 * t6730 * t23495 + t83004 / 576.0_f64 - t23419 * t10419 / 384.0_f64 + t83008 * t3073 / 384.0_f64 + t23419 * t10394 / 768.0_f64 + t23419 * t10398 / 768.0_f64 + t83016 * t10405 / 384.0_f64 - 0.24223653656484234513e-2_f64 * t6722 * t23413 * t1937 + 0.21801288290835811062e-1_f64 * t23453 * t6729 * t1937 + t83025 / 54.0_f64 + t83028;
    t83029
}
