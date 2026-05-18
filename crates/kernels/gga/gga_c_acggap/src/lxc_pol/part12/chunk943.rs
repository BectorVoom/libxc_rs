//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 943/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk943<F: Float>(t1072: F, t429: F, t7507: F, t7512: F, t310: F, t7506: F, t7514: F, t7518: F, t22: F, t30174: F, t420: F, t56: F) -> (F, F, F, F) {
    let t31386 = t7507 * t7512 * t429 * t1072;
    let t31388 = t310 * t7506;
    let t31389 = t31388 * t7514;
    let t31391 = t31388 * t7518;
    let t31402 = F::new(1.0) / t22 / t30174;
    let t31404 = t31402 * t56 * t420;
    (t31386, t31389, t31391, t31404)
}
