//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1149/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1149<F: Float>(t20561: F, t20671: F, t31047: F, t1406: F, t6582: F, t9268: F, t9271: F, t9274: F, t1265: F, t2487: F, t9438: F, t9448: F) -> (F, F, F, F, F, F) {
    let t31050 = F::new(0.85206502119823888169e0) * t31047 * t20671 * t20561;
    let t31051 = t1406 * t6582;
    let t31053 = F::new(0.38342925953920749676e1) * t31051 * t9268;
    let t31054 = t1406 * t9271;
    let t31056 = F::new(0.23005755572352449806e1) * t31054 * t9274;
    let t31065 = t2487 * t9438 * t9448 * t1265;
    (t31050, t31051, t31053, t31054, t31056, t31065)
}
