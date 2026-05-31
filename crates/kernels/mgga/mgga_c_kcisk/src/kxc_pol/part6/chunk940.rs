//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 940/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk940<F: Float>(t29402: F, t29539: F, t29583: F, t29625: F, t1908: F, t2604: F, t9108: F, t5400: F, t12042: F, t15989: F, t16389: F, t22564: F, t22575: F, t22583: F, t22698: F, t22705: F, t22707: F, t28362: F, t28379: F, t28387: F, t28394: F, t28404: F) -> (F, F, F, F) {
    let t29627 = t29402 + t29539 + t29583 + t29625;
    let t29628 = t1908 * t29627;
    let t29636 = t9108 * t2604;
    let t29637 = t29636 * t5400;
    let t29653 = -F::cast_from(0.68863333333333333332e0_f64) * t15989 + F::cast_from(0.94674375e0_f64) * t28362 + F::cast_from(0.3529725e1_f64) * t28394 - t12042 - F::cast_from(0.34731666666666666667e0_f64) * t16389 + F::cast_from(0.69463333333333333335e-1_f64) * t22698 + F::cast_from(0.34431666666666666666e0_f64) * t22564 - F::cast_from(0.103295e1_f64) * t22575 + F::cast_from(0.51647499999999999999e0_f64) * t22583 - F::cast_from(0.41678000000000000001e0_f64) * t22705 + F::cast_from(0.20839e0_f64) * t22707 + F::cast_from(0.62517e0_f64) * t28404 - F::cast_from(0.103295e1_f64) * t28379 + F::cast_from(0.309885e1_f64) * t28387;
    (t29628, t29636, t29637, t29653)
}
