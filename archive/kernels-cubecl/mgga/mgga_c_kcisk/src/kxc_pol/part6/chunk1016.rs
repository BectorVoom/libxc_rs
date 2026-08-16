//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1016/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1016<F: Float>(t30558: F, t3679: F, t12910: F, t12884: F, t30551: F, t12888: F, t13064: F, t3725: F, t19100: F, t25590: F, t25601: F, t25609: F, t25696: F, t25699: F, t25701: F, t30569: F, t30572: F, t30582: F, t30585: F, t30606: F, t30608: F, t30610: F) -> (F, F, F, F) {
    let t30666 = t30558 * t3679;
    let t30668 = F::cast_from(0.96490945932906628932e2_f64) * t12910 * t30666;
    let t30669 = t12884 * t30551;
    let t30670 = t30669 * t12888;
    let t30673 = t13064 * t30551;
    let t30674 = t30673 * t3725;
    let t30691 = -F::cast_from(0.60384999999999999999e0_f64) * t30569 + F::cast_from(0.181155e1_f64) * t30572 - F::cast_from(0.40256666666666666668e0_f64) * t19100 + F::cast_from(0.20128333333333333333e0_f64) * t25590 - F::cast_from(0.60385000000000000001e0_f64) * t25601 + F::cast_from(0.30192500000000000001e0_f64) * t25609 - F::cast_from(0.33114e0_f64) * t25696 + F::cast_from(0.16557e0_f64) * t25699 + F::cast_from(0.5519e-1_f64) * t25701 - F::cast_from(0.82785e-1_f64) * t30582 + F::cast_from(0.49671e0_f64) * t30585 + F::cast_from(0.16504875e0_f64) * t30606 + F::cast_from(0.258925e1_f64) * t30608 - F::cast_from(0.3883875e1_f64) * t30610;
    (t30668, t30670, t30674, t30691)
}
