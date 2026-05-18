//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 841/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk841<F: Float>(t11271: F, t2268: F, t2349: F, t11187: F, t2317: F, t6525: F, t11254: F, t2293: F, t2343: F, t11259: F, t6320: F, t13262: F, t6305: F) -> (F, F, F, F, F, F, F) {
    let t44457 = F::new(0.85365019907028448797e-1) * t2268 * t11271 * t2349;
    let t44468 = t6525 * t11187 * t2317;
    let t44469 = F::new(0.11856252764865062333e-2) * t44468;
    let t44470 = t11254 * t2293;
    let t44473 = F::new(0.56910013271352299198e-1) * t2268 * t2343 * t44470;
    let t44474 = t11259 * t2293;
    let t44477 = F::new(0.17073003981405689759e0) * t2268 * t6320 * t44474;
    let t44479 = F::new(0.56910013271352299198e-1) * t6305 * t13262;
    (t44457, t44469, t44470, t44473, t44474, t44477, t44479)
}
