//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 307/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk307<F: Float>(t11: F, t19: F, t1154: F, t1161: F, t357: F, t21: F, t410: F, t1165: F, t1167: F, t1169: F, t363: F, t347: F) -> (F, F, F, F) {
    let t1195 = F::new(1.0)/f64::sqrt(t11);
    let t1196 = t1195 * t19;
    let t1197 = t1196 * t1154;
    let t1199 = t357 * t1161;
    let t1201 = t21 * t410;
    let t1203 = -F::new(0.42198333333333333333e0) * t1165 + F::new(0.84396666666666666666e0) * t1167 + F::new(0.39862222222222222223e0) * t1169 + F::new(0.68258333333333333333e-1) * t1197 + F::new(0.13651666666666666667e0) * t1199 + F::new(0.13692777777777777778e0) * t1201;
    let t1204 = t1203 * t363;
    let t1206 = F::new(1.0) * t347 * t1204;
    (t1197, t1199, t1201, t1206)
}
