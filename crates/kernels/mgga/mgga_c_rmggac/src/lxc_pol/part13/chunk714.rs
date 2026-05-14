//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 714/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk714<F: Float>(t7229: F, t7243: F, t7457: F, t2186: F, t7424: F, t7404: F, t7421: F, t1986: F, t1995: F, t305: F, t321: F, t2134: F, t27: F, t3118: F, t36271: F, t7204: F) -> (F, F, F, F, F, F, F, F) {
    let t36343 = t7229 * t7243;
    let t36344 = t36343 * t7457;
    let t36379 = t2186 * t7424;
    let t36381 = t2186 * t7404;
    let t36383 = t2186 * t7421;
    let t36391 = t1986 * t305 * t1995 * t321;
    let t36402 = t2134 * t27 * t3118 * t321;
    let t36416 = t7204 * t36271;
    (t36343, t36344, t36379, t36381, t36383, t36391, t36402, t36416)
}
