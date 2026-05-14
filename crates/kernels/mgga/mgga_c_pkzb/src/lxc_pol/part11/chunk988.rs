//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 988/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk988<F: Float>(t1835: F, t87: F, t5829: F, t690: F, t1731: F, t218: F, t220: F, t5555: F, t679: F, t16194: F, t213: F, t778: F, t210: F, t17348: F, t1975: F, t252: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t17359 = t1835 * t1835;
    let t17361 = 1.0 / t87 / t17359;
    let t17391 = t690 * t5829;
    let t17402 = t218 * t1731 * t220;
    let t17403 = 0.13490888888888888889e1 * t17402;
    let t17405 = t218 * t5555 * t679;
    let t17432 = 1.0 / t213 / t16194 / t778 / 96.0;
    let t17444 = f64::powf(t210, -0.25e1);
    let t17454 = 0.31310740740740740741e1 * t17348;
    let t17455 = 280.0 / 81.0 * t17348;
    let t17473 = t1975 * t1975;
    let t17474 = 1.0 / t17473;
    let t17475 = t252 * t17474;
    (t17361, t17391, t17402, t17403, t17405, t17432, t17444, t17454, t17455, t17474, t17475)
}
