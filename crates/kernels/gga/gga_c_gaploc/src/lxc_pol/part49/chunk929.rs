//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 929/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk929<F: Float>(t2487: F, t41965: F, t6711: F, t204: F, t2476: F, t41839: F, t40228: F, t40234: F, t40237: F, t40239: F, t40243: F, t40249: F) -> (F, F, F, F, F, F, F, F) {
    let t41968 = F::new(0.43710935587469654631e2) * t2487 * t6711 * t41965;
    let t41970 = t2476 * t204 * t41839;
    let t41972 = F::new(0.29792074959875355558e-1) * t40228;
    let t41973 = F::new(0.89376224879626066674e-1) * t40234;
    let t41974 = F::new(0.59584149919750711116e-1) * t40237;
    let t41975 = F::new(0.29792074959875355558e-1) * t40239;
    let t41976 = F::new(0.29792074959875355558e-1) * t40243;
    let t41978 = F::new(0.17041300423964777634e0) * t40249;
    (t41968, t41970, t41972, t41973, t41974, t41975, t41976, t41978)
}
