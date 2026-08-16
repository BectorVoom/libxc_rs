//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1190/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1190<F: Float>(t36286: F, t36236: F, t36238: F, t36240: F, t36243: F, t36246: F, t36250: F, t36253: F, t36256: F, t36259: F, t36262: F, t36266: F, t36269: F, t36274: F, t36276: F, t36279: F, t36284: F) -> F {
    let t36287 = F::cast_from(0.17149607247227894789e-1_f64) * t36286;
    let t36288 = F::cast_from(0.22675591804667994221e-1_f64) * t36236 - F::cast_from(0.95275595817932748827e-3_f64) * t36238 - F::cast_from(0.80031500487063509014e-2_f64) * t36240 + F::cast_from(0.31448092289604152068e-2_f64) * t36243 - F::cast_from(0.94344276868812456204e-3_f64) * t36246 - F::cast_from(0.18868855373762491241e-2_f64) * t36250 - t36253 / F::cast_from(24.0_f64) + t36256 / F::cast_from(128.0_f64) + t36259 / F::cast_from(96.0_f64) + t36262 / F::cast_from(192.0_f64) + F::cast_from(0.114609375e-1_f64) * t36266 + F::cast_from(0.7640625e-2_f64) * t36269 + t36274 + F::cast_from(0.21437009059034868486e-2_f64) * t36276 + F::cast_from(0.12862205435420921092e-2_f64) * t36279 + t36284 - t36287;
    t36288
}
