//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 724/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk724<F: Float>(t34688: F, t9272: F, t9273: F, t18313: F, t31119: F, t3394: F, t35180: F, t9562: F, t10256: F, t30830: F, t913: F, t12957: F, t31356: F, t35216: F, t9287: F, t2875: F, t4386: F, t544: F, t9078: F) -> (F, F, F, F, F, F, F) {
    let t41656 = t9272 * t34688 * t9273;
    let t41657 = 0.10352590007558602413e2 * t41656;
    let t41660 = t31119 * t18313 * t3394 * t9273;
    let t41661 = 0.46011511144704899612e1 * t41660;
    let t41666 = t35180 * t9562;
    let t41667 = 0.20854452471912748891e0 * t41666;
    let t41669 = t30830 * t913 * t10256;
    let t41670 = 0.59584149919750711116e-1 * t41669;
    let t41674 = t31356 * t12957;
    let t41676 = t35216 * t9287;
    let t41677 = 0.29792074959875355558e-1 * t41676;
    let t41681 = 0.27805936629216998521e0 * t544 * t9078 * t2875 * t4386;
    (t41657, t41661, t41667, t41670, t41674, t41677, t41681)
}
