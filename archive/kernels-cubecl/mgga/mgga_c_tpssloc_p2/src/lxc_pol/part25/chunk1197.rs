//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1197/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1197<F: Float>(t81146: F, t81153: F, t12168: F, t1336: F, t7208: F, t81115: F, t81122: F, t81125: F, t81127: F, t81132: F, t81140: F, t81149: F, t81157: F, t81160: F, t81165: F, t81169: F, t81173: F, t81177: F, t81181: F, t81184: F) -> F {
    let t84595 = F::cast_from(0.27415567780803773942e-2_f64) * t81146;
    let t84597 = F::cast_from(0.19739208802178717238e0_f64) * t81153;
    let t84606 = F::cast_from(0.24674011002723396548e-1_f64) * t81115 - F::cast_from(0.49348022005446793095e-1_f64) * t81122 + F::cast_from(0.24674011002723396548e-1_f64) * t81125 + F::cast_from(0.23029076935875170111e0_f64) * t81127 - F::cast_from(0.9869604401089358619e-1_f64) * t81132 - t1336 * t7208 * t12168 - F::cast_from(0.14804406601634037928e0_f64) * t81140 - t84595 - F::cast_from(0.49348022005446793095e-1_f64) * t81149 + t84597 + F::cast_from(0.16449340668482264365e-1_f64) * t81157 - F::cast_from(0.46058153871750340221e0_f64) * t81160 - F::cast_from(0.29608813203268075857e0_f64) * t81165 + F::cast_from(0.9869604401089358619e-1_f64) * t81169 + F::cast_from(0.9869604401089358619e-1_f64) * t81173 - F::cast_from(0.16449340668482264365e-1_f64) * t81177 + F::cast_from(0.9869604401089358619e-1_f64) * t81181 - F::cast_from(0.23029076935875170111e0_f64) * t81184;
    t84606
}
