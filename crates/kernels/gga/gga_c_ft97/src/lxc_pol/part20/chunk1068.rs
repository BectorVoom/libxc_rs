//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1068/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1068<F: Float>(t27836: F, t684: F, t24432: F, t6118: F, t1091: F, t24477: F, t96934: F, t96935: F, t1131: F, t24437: F, t2514: F, t2574: F, t6119: F, t27819: F, t53307: F, t729: F) -> (F, F, F, F, F) {
    let t108218 = t27836 * t684;
    let t108220 = t6118 * t24432 * t108218;
    let t108224 = t96934 * t96935 * t1091 * t24477;
    let t108229 = t24437 * t2574 * t6119 * t1131 * t2514;
    let t108233 = t27819 * t729 * t6119 * t53307;
    (t108218, t108220, t108224, t108229, t108233)
}
