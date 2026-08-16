//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1190/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1190(t36286: f64, t36236: f64, t36238: f64, t36240: f64, t36243: f64, t36246: f64, t36250: f64, t36253: f64, t36256: f64, t36259: f64, t36262: f64, t36266: f64, t36269: f64, t36274: f64, t36276: f64, t36279: f64, t36284: f64) -> f64 {
    let t36287 = 0.17149607247227894789e-1_f64 * t36286;
    let t36288 = 0.22675591804667994221e-1_f64 * t36236 - 0.95275595817932748827e-3_f64 * t36238 - 0.80031500487063509014e-2_f64 * t36240 + 0.31448092289604152068e-2_f64 * t36243 - 0.94344276868812456204e-3_f64 * t36246 - 0.18868855373762491241e-2_f64 * t36250 - t36253 / 24.0_f64 + t36256 / 128.0_f64 + t36259 / 96.0_f64 + t36262 / 192.0_f64 + 0.114609375e-1_f64 * t36266 + 0.7640625e-2_f64 * t36269 + t36274 + 0.21437009059034868486e-2_f64 * t36276 + 0.12862205435420921092e-2_f64 * t36279 + t36284 - t36287;
    t36288
}
