//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1046/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1046<F: Float>(t4680: F, t7564: F, t8613: F, t1181: F, t4718: F, t604: F, t7426: F, t31349: F, t3360: F, t4284: F, t36236: F, t36238: F, t36240: F, t36243: F, t36246: F, t36250: F, t36253: F, t36256: F, t36259: F, t36262: F, t36266: F, t36269: F, t36274: F, t36276: F) -> (F,) {
    let t36279 = t7564 * t4680 * t8613;
    let t36283 = t7426 * t1181 * t604 * t4718;
    let t36284 = 0.42874018118069736972e-3 * t36283;
    let t36286 = t3360 * t31349 * t4284;
    let t36287 = 0.17149607247227894789e-1 * t36286;
    let t36288 = 0.22675591804667994221e-1 * t36236 - 0.95275595817932748827e-3 * t36238 - 0.80031500487063509014e-2 * t36240 + 0.31448092289604152068e-2 * t36243 - 0.94344276868812456204e-3 * t36246 - 0.18868855373762491241e-2 * t36250 - t36253 / 24.0 + t36256 / 128.0 + t36259 / 96.0 + t36262 / 192.0 + 0.114609375e-1 * t36266 + 0.7640625e-2 * t36269 + t36274 + 0.21437009059034868486e-2 * t36276 + 0.12862205435420921092e-2 * t36279 + t36284 - t36287;
    (t36288,)
}
