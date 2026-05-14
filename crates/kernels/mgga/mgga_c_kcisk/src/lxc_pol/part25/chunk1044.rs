//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1044/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1044<F: Float>(t4781: F, t7509: F, t10699: F, t2394: F, t4762: F, t2605: F, t5374: F, t2386: F, t4761: F, t2597: F, t5372: F, t12095: F, t12098: F, t12105: F, t16459: F, t16540: F, t16543: F, t18601: F, t18604: F, t18607: F, t18626: F, t5375: F, t5398: F, t5408: F, t5409: F, t5415: F, t7510: F, t764: F) -> (F,) {
    let t18630 = t7509 * t4781;
    let t18633 = t2394 * t10699;
    let t18634 = t18633 * t4762;
    let t18637 = t2605 * t5374;
    let t18640 = t2386 * t4761;
    let t18643 = t2597 * t5372;
    let t18646 = 0.34631511798751726598e2 * t12095 * t7510 - 0.23392893589820816284e1 * t5408 * t18601 - 0.11696446794910408142e1 * t5408 * t18604 - 0.1038945353962551798e3 * t12098 * t18607 - 0.3109e-1 * t18626 * t764 + t16540 + t16543 - 0.19751789702565206229e-1 * t16459 + 0.17315755899375863299e2 * t5415 * t18630 + 0.1025389702100779493e4 * t12105 * t18634 + 6.0 * t5398 * t18637 - 0.11696446794910408142e1 * t18640 * t5409 - 2.0 * t18643 * t5375;
    (t18646,)
}
