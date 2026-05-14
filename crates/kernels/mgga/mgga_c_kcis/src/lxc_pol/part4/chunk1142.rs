//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1142/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1142<F: Float>(t1482: F, t16533: F, t542: F, t1477: F, t16194: F, t3255: F, t5432: F, t5436: F, t5442: F, t1419: F, t5808: F, t5498: F, t11671: F, t544: F, t16055: F, t1102: F, t11379: F, t11381: F, t11384: F, t11640: F, t11642: F, t11644: F, t16457: F, t16461: F, t16464: F, t16467: F, t16470: F, t16474: F, t16480: F, t344: F, t4587: F) -> (F, F, F, F) {
    let t16534 = t1482 * t16533;
    let t16535 = t542 * t16534;
    let t16538 = t1477 * t16194;
    let t16539 = t542 * t16538;
    let t16543 = 0.13140859333333333334e-2 * t3255 * t5432;
    let t16545 = 0.8760572888888888889e-3 * t3255 * t5436;
    let t16547 = 0.17521145777777777778e-2 * t3255 * t5442;
    let t16548 = t5808 * t1419;
    let t16549 = t5498 * t16548;
    let t16552 = t11671 * t544;
    let t16553 = t16552 * t16055;
    let t16559 = -0.65704296666666666667e-3 * t1102 * t16457 + 0.26281718666666666666e-2 * t4587 * t16461 - 0.13140859333333333333e-2 * t1102 * t16464 + 0.52563437333333333332e-2 * t4587 * t16467 - 0.65704296666666666666e-2 * t1102 * t16470 + 0.492782225e-3 * t1102 * t16474 + 0.13140859333333333334e-2 * t11379 - 0.8760572888888888889e-3 * t11381 + 0.13140859333333333333e-2 * t1102 * t16480 - 0.98556445e-3 * t344 * t16535 + t11384 + 0.1478346675e-2 * t344 * t16539 + t16543 - t16545 - t16547 + 0.39422578e-2 * t1102 * t16549 + 0.29201909629629629629e-2 * t1102 * t16553 - 0.2920190962962962963e-3 * t11640 + 0.43802864444444444445e-3 * t11642 + 0.73004774074074074075e-3 * t11644;
    (t16534, t16538, t16548, t16559)
}
