//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 728/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk728<F: Float>(t2268: F, t38184: F, t888: F, t2349: F, t3565: F, t11264: F, t6767: F, t3516: F, t4538: F, t6759: F, t11254: F, t2343: F, t6509: F, t11408: F, t6320: F, t13265: F, t484: F) -> (F, F, F, F, F, F, F) {
    let t44355 = 0.85365019907028448797e-1 * t2268 * t38184 * t888;
    let t44358 = 0.19918504644973304719e0 * t2268 * t3565 * t2349;
    let t44363 = 0.14227503317838074799e1 * t2268 * t11264 * t6767;
    let t44364 = t4538 * t3516;
    let t44367 = 0.17073003981405689759e1 * t2268 * t44364 * t6759;
    let t44371 = 0.34146007962811379518e0 * t2268 * t2343 * t11254 * t6509;
    let t44375 = 0.17073003981405689759e0 * t2268 * t6320 * t11408 * t6509;
    let t44376 = t484 * t13265;
    (t44355, t44358, t44363, t44367, t44371, t44375, t44376)
}
