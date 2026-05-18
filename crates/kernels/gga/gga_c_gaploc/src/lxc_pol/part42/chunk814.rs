//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 814/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk814<F: Float>(t11264: F, t2268: F, t6767: F, t3516: F, t4538: F, t6759: F, t11254: F, t2343: F, t6509: F, t11408: F, t6320: F, t13265: F, t484: F) -> (F, F, F, F, F) {
    let t44363 = F::new(0.14227503317838074799e1) * t2268 * t11264 * t6767;
    let t44364 = t4538 * t3516;
    let t44367 = F::new(0.17073003981405689759e1) * t2268 * t44364 * t6759;
    let t44371 = F::new(0.34146007962811379518e0) * t2268 * t2343 * t11254 * t6509;
    let t44375 = F::new(0.17073003981405689759e0) * t2268 * t6320 * t11408 * t6509;
    let t44376 = t484 * t13265;
    (t44363, t44367, t44371, t44375, t44376)
}
