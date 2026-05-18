//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 308/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk308<F: Float>(t1227: F, t372: F, t1165: F, t1167: F, t1169: F, t1197: F, t1199: F, t1201: F, t374: F, t381: F, t373: F, t1225: F) -> (F, F, F, F, F, F) {
    let t1229 = F::new(0.11696446794910408142e1) * t372 * t1227;
    let t1236 = -F::new(0.57538888888888888889e0) * t1165 + F::new(0.11507777777777777778e1) * t1167 + F::new(0.40256666666666666667e0) * t1169 + F::new(0.366775e-1) * t1197 + F::new(0.73355e-1) * t1199 + F::new(0.137975e0) * t1201;
    let t1238 = t374 * t1236 * t381;
    let t1240 = F::new(0.58482233974552040708e0) * t372 * t1238;
    let t1241 = t373 * t373;
    let t1242 = F::new(1.0) / t1241;
    let t1243 = t1242 * t1225;
    (t1229, t1236, t1238, t1240, t1242, t1243)
}
