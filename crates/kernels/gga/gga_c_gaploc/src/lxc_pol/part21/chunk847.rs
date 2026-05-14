//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 847/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk847<F: Float>(t2293: F, t2366: F, t2365: F, t1429: F, t6696: F, t901: F, t6700: F, t3162: F, t549: F, t2372: F, t2389: F, t3248: F, t731: F, t3240: F, t2549: F, t7221: F, t883: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t9572 = t2366 * t2293;
    let t9573 = t2365 * t9572;
    let t9575 = 0.29792074959875355558e-1 * t1429 * t9573;
    let t9577 = 0.29792074959875355558e-1 * t6696 * t901;
    let t9579 = 0.29792074959875355558e-1 * t6700 * t901;
    let t9580 = t549 * t3162;
    let t9582 = 0.59584149919750711116e-1 * t1429 * t9580;
    let t9584 = 0.59584149919750711116e-1 * t2372 * t2389;
    let t9618 = 0.85450291446024714264e-3 * t731 * t3248;
    let t9620 = 0.85450291446024714264e-3 * t731 * t3240;
    let t9622 = 0.64087718584518535698e-3 * t2549 * t3248;
    let t9624 = t883 * t7221;
    (t9572, t9573, t9575, t9577, t9579, t9580, t9582, t9584, t9618, t9620, t9622, t9624)
}
