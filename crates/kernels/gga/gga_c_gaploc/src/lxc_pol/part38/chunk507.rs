//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 507/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk507<F: Float>(t2365: F, t9572: F, t1429: F, t6696: F, t901: F, t6700: F, t3162: F, t549: F, t2372: F, t2389: F, t3248: F, t731: F) -> (F, F, F, F, F, F) {
    let t9573 = t2365 * t9572;
    let t9575 = F::new(0.29792074959875355558e-1) * t1429 * t9573;
    let t9577 = F::new(0.29792074959875355558e-1) * t6696 * t901;
    let t9579 = F::new(0.29792074959875355558e-1) * t6700 * t901;
    let t9580 = t549 * t3162;
    let t9582 = F::new(0.59584149919750711116e-1) * t1429 * t9580;
    let t9584 = F::new(0.59584149919750711116e-1) * t2372 * t2389;
    let t9618 = F::new(0.85450291446024714264e-3) * t731 * t3248;
    (t9575, t9577, t9579, t9582, t9584, t9618)
}
