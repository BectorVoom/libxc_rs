//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1108/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1108<F: Float>(t23052: F, t93506: F, t2: F, t22862: F, t22955: F, t22960: F, t23054: F, t1317: F, t23019: F, t376: F, t22976: F, t22999: F, t5665: F, t22971: F, t23193: F, t89: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t93507 = t93506 * t23052;
    let t93508 = t93507 / 3.0;
    let t93509 = t2 * t22862;
    let t93521 = t93506 * t22955;
    let t93522 = t93521 / 18.0;
    let t93523 = t23054 * t22960;
    let t93524 = 2.0 / 9.0 * t93523;
    let t93530 = t1317 * t376 * t23019;
    let t93541 = t1317 * t376 * t22976;
    let t93542 = t93541 / 3.0;
    let t93557 = t5665 * t376 * t22999;
    let t93558 = t93557 / 6.0;
    let t93560 = t1317 * t376 * t22971;
    let t93561 = 2.0 / 3.0 * t93560;
    let t93577 = t89 * t376 * t23193;
    (t93507, t93508, t93509, t93521, t93522, t93523, t93524, t93530, t93541, t93542, t93557, t93558, t93560, t93561, t93577)
}
