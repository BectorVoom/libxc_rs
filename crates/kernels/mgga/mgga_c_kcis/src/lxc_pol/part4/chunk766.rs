//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 766/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk766<F: Float>(t1220: F, t5233: F, t1830: F, t3577: F, t1219: F, t1684: F, t962: F, t1835: F, t969: F, t4758: F, t971: F, t1692: F, t3034: F, t1212: F, t1221: F, t1225: F, t1226: F, t1831: F, t3545: F, t3550: F, t3575: F, t3582: F, t3585: F, t3592: F, t405: F, t4684: F, t4687: F, t4689: F, t4692: F, t4721: F, t4725: F, t4732: F, t5208: F, t5211: F, t5216: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5234 = t5233 * t1220;
    let t5237 = t1830 * t3577;
    let t5238 = t5237 * t1219;
    let t5242 = t1684 * t962;
    let t5247 = t1835 * t969;
    let t5250 = t4758 * t971;
    let t5253 = t1692 * t3034;
    let t5254 = t5253 * t969;
    let t5257 = -0.3109e-1 * t5208 * t405 + 1.0 * t5211 * t1221 + 1.0 * t3545 * t1831 - 2.0 * t3550 * t5216 + 1.0 * t1212 * t5234 + 0.32164683177870697974e2 * t3575 * t5238 + t4684 - t4687 - t4689 + t4692 - t4721 - t4725 - 0.19751789702565206229e-1 * t4732 + 0.58482233974552040708e0 * t5242 * t1226 + 0.58482233974552040708e0 * t3582 * t1835 - 0.11696446794910408142e1 * t3585 * t5247 + 0.58482233974552040708e0 * t1225 * t5250 + 0.17315755899375863299e2 * t3592 * t5254;
    (t5234, t5237, t5238, t5242, t5247, t5250, t5253, t5254, t5257)
}
