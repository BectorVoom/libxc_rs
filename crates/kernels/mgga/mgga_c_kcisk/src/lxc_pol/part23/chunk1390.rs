//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1390/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1390<F: Float>(t21499: F, t33372: F, t110029: F, t110037: F, t114628: F, t114631: F, t114634: F, t114636: F, t114638: F, t114643: F, t114651: F, t19087: F, t19972: F, t20053: F, t20067: F, t20116: F, t2718: F, t32008: F, t32087: F, t32088: F, t32090: F, t33408: F, t33415: F, t82257: F, t9438: F) -> (F, F) {
    let t114664 = t33372 * t21499;
    let t114669 = 0.88437037037037037034e-2 * t114628 + 0.16581944444444444444e-2 * t114631 + t114634 + t114636 - t114638 + 0.55555555555555555558e-1 * t19972 * t9438 * t2718 - 0.22109259259259259258e-2 * t114643 + 0.13888888888888888889e-1 * t32087 * t20067 * t32088 * t82257 + 0.13888888888888888889e-1 * t32087 * t114651 + 0.27777777777777777778e-1 * t32087 * t20053 * t33408 * t19087 + 0.53611111111111111112e-2 * t32008 * t114651 - 0.18518518518518518519e-1 * t32087 * t20116 * t33415 * t19087 + 0.69444444444444444446e-2 * t114664 * t32090 - 0.3684876543209876543e-3 * t110029 + 0.33163888888888888888e-2 * t110037;
    (t114664, t114669)
}
