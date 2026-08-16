//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 891/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk891(t360: f64, t9543: f64, t2591: f64, t8820: f64, t3105: f64, t6118: f64, t3186: f64, t5119: f64, t514: f64, t2122: f64, t2557: f64, t2598: f64, t2600: f64, t6139: f64, t8029: f64, t8275: f64, t8277: f64, t8282: f64, t8284: f64, t9502: f64, t9509: f64, t9513: f64, t9517: f64, t9521: f64, t9526: f64, t9530: f64, t9536: f64, t9540: f64) -> (f64, f64) {
    let t9544 = t360 * t9543;
    let t9547 = t8820 * t2591;
    let t9548 = t360 * t9547;
    let t9551 = t6118 * t3105;
    let t9553 = t5119 * t3186;
    let t9554 = t514 * t9553;
    let t9556 = -0.2600466522016280569e0_f64 * t6139 * t9502 - 0.1358426014257923078e0_f64 * t8275 - 0.65854491829355115985e-1_f64 * t8277 + 0.17336443480108537126e0_f64 * t2598 * t9509 - 0.10975748638225852664e0_f64 * t2122 * t9513 + 0.10975748638225852664e0_f64 * t2557 * t9517 + 0.17336443480108537126e0_f64 * t9521 * t2600 + 0.54878743191129263322e-1_f64 * t2557 * t9526 + 0.86682217400542685632e-1_f64 * t2598 * t9530 - 0.16463622957338778997e0_f64 * t2557 * t9536 + 0.16463622957338778996e0_f64 * t2557 * t9540 - 0.2600466522016280569e0_f64 * t8029 * t9544 + 0.26004665220162805689e0_f64 * t2598 * t9548 - 0.25610080155860322883e0_f64 * t9551 + t8282 + 0.29272321618148349057e-1_f64 * t9554 - t8284;
    (t9547, t9556)
}
