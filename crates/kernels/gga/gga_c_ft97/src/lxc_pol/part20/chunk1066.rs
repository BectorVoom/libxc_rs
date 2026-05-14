//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1066/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1066<F: Float>(t2492: F, t6119: F, t1154: F, t668: F, t108179: F, t108186: F, t2601: F, t96934: F, t96935: F, t24438: F, t2476: F, t27819: F, t6135: F, t992: F, t24437: F, t27850: F, t684: F) -> (F, F, F, F, F) {
    let t108187 = t2492 * t6119;
    let t108188 = t1154 * t668;
    let t108191 = t108186 * t108187 * t108188 * t108179;
    let t108195 = t96934 * t96935 * t108188 * t2601;
    let t108200 = t27819 * t24438 * t6135 * t992 * t2476;
    let t108204 = t24437 * t24438 * t27850 * t684;
    (t108187, t108191, t108195, t108200, t108204)
}
