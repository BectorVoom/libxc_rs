//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 721/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk721<F: Float>(t11187: F, t2317: F, t6525: F, t11254: F, t2293: F, t2268: F, t2343: F, t11259: F, t6320: F, t13262: F, t6305: F, t36178: F, t874: F, t13268: F, t6313: F, t13327: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t44468 = t6525 * t11187 * t2317;
    let t44469 = 0.11856252764865062333e-2 * t44468;
    let t44470 = t11254 * t2293;
    let t44473 = 0.56910013271352299198e-1 * t2268 * t2343 * t44470;
    let t44474 = t11259 * t2293;
    let t44477 = 0.17073003981405689759e0 * t2268 * t6320 * t44474;
    let t44479 = 0.56910013271352299198e-1 * t6305 * t13262;
    let t44480 = t36178 * t874;
    let t44483 = 0.56910013271352299198e-1 * t2268 * t2343 * t44480;
    let t44485 = 0.45528010617081839357e0 * t6313 * t13268;
    let t44487 = 0.28455006635676149599e-1 * t6305 * t13327;
    (t44469, t44470, t44473, t44474, t44477, t44479, t44480, t44483, t44485, t44487)
}
