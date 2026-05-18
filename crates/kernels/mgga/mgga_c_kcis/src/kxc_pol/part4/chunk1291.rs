//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1291/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1291<F: Float>(t11671: F, t544: F, t16055: F, t1102: F, t11379: F, t11381: F, t11384: F, t11640: F, t11642: F, t11644: F, t16457: F, t16461: F, t16464: F, t16467: F, t16470: F, t16474: F, t16480: F, t16535: F, t16539: F, t16543: F, t16545: F, t16547: F, t16549: F, t344: F, t4587: F) -> F {
    let t16552 = t11671 * t544;
    let t16553 = t16552 * t16055;
    let t16559 = -F::new(0.65704296666666666667e-3) * t1102 * t16457 + F::new(0.26281718666666666666e-2) * t4587 * t16461 - F::new(0.13140859333333333333e-2) * t1102 * t16464 + F::new(0.52563437333333333332e-2) * t4587 * t16467 - F::new(0.65704296666666666666e-2) * t1102 * t16470 + F::new(0.492782225e-3) * t1102 * t16474 + F::new(0.13140859333333333334e-2) * t11379 - F::new(0.8760572888888888889e-3) * t11381 + F::new(0.13140859333333333333e-2) * t1102 * t16480 - F::new(0.98556445e-3) * t344 * t16535 + t11384 + F::new(0.1478346675e-2) * t344 * t16539 + t16543 - t16545 - t16547 + F::new(0.39422578e-2) * t1102 * t16549 + F::new(0.29201909629629629629e-2) * t1102 * t16553 - F::new(0.2920190962962962963e-3) * t11640 + F::new(0.43802864444444444445e-3) * t11642 + F::new(0.73004774074074074075e-3) * t11644;
    t16559
}
