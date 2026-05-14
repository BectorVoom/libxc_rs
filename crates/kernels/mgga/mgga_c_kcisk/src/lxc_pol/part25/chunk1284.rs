//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1284/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1284<F: Float>(t32955: F, t34073: F, t2469: F, t33003: F, t5043: F, t7261: F, t15905: F, t6713: F, t9679: F, t15910: F, t15917: F, t60514: F, t112221: F, t4824: F, t112226: F, t116120: F, t32917: F, t32938: F, t32942: F, t32990: F, t34078: F, t34122: F, t34192: F, t9649: F, t9667: F) -> (F, F, F, F, F, F) {
    let t116245 = t34073 * t32955;
    let t116251 = t7261 * t33003 * t2469 * t5043;
    let t116258 = t6713 * t9679 * t15905;
    let t116269 = t6713 * t9679 * t15910;
    let t116272 = t60514 * t9679 * t15917;
    let t116276 = t7261 * t112221 * t2469 * t4824;
    let t116279 = -0.23148148148148148148e-2 * t116245 + 0.69444444444444444446e-2 * t34073 * t32917 - 0.120625e-1 * t9649 * t116251 - 0.22109259259259259258e-2 * t112226 - 0.69444444444444444446e-2 * t116120 * t9667 - 0.44218518518518518517e-2 * t116258 - 0.20833333333333333334e-1 * t34122 * t32938 - 0.8041666666666666667e-2 * t34192 * t32938 - 0.41666666666666666668e-1 * t32942 * t34078 - 0.41666666666666666668e-1 * t32990 * t34078 + 0.13265555555555555555e-1 * t116269 - 0.11054629629629629629e-1 * t116272 + 0.44229166666666666667e-1 * t9649 * t116276;
    (t116251, t116258, t116269, t116272, t116276, t116279)
}
