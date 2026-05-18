//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 892/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk892<F: Float>(t360: F, t9543: F, t2591: F, t8820: F, t3105: F, t6118: F, t3186: F, t5119: F, t514: F, t2122: F, t2557: F, t2598: F, t2600: F, t6139: F, t8029: F, t8275: F, t8277: F, t8282: F, t8284: F, t9502: F, t9509: F, t9513: F, t9517: F, t9521: F, t9526: F, t9530: F, t9536: F, t9540: F) -> (F, F) {
    let t9544 = t360 * t9543;
    let t9547 = t8820 * t2591;
    let t9548 = t360 * t9547;
    let t9551 = t6118 * t3105;
    let t9553 = t5119 * t3186;
    let t9554 = t514 * t9553;
    let t9556 = -F::new(0.2600466522016280569e0) * t6139 * t9502 - F::new(0.1358426014257923078e0) * t8275 - F::new(0.65854491829355115985e-1) * t8277 + F::new(0.17336443480108537126e0) * t2598 * t9509 - F::new(0.10975748638225852664e0) * t2122 * t9513 + F::new(0.10975748638225852664e0) * t2557 * t9517 + F::new(0.17336443480108537126e0) * t9521 * t2600 + F::new(0.54878743191129263322e-1) * t2557 * t9526 + F::new(0.86682217400542685632e-1) * t2598 * t9530 - F::new(0.16463622957338778997e0) * t2557 * t9536 + F::new(0.16463622957338778996e0) * t2557 * t9540 - F::new(0.2600466522016280569e0) * t8029 * t9544 + F::new(0.26004665220162805689e0) * t2598 * t9548 - F::new(0.25610080155860322883e0) * t9551 + t8282 + F::new(0.29272321618148349057e-1) * t9554 - t8284;
    (t9547, t9556)
}
