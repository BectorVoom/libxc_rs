//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 789/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk789<F: Float>(t10608: F, t9272: F, t9278: F, t34600: F, t544: F, t9287: F, t34604: F, t12938: F, t2464: F, t587: F, t40517: F, t26435: F, t6710: F, t9438: F) -> (F, F, F, F, F, F) {
    let t42349 = t9272 * t10608 * t9278;
    let t42366 = t544 * t34600 * t9287;
    let t42369 = t544 * t34604 * t9287;
    let t42378 = t587 * t2464 * t12938;
    let t42381 = F::new(0.25561950635947166451e0) * t40517;
    let t42400 = t6710 * t9438 * t26435;
    (t42349, t42366, t42369, t42378, t42381, t42400)
}
