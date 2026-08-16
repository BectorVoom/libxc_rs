//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 965/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk965<F: Float>(t41884: F, t11549: F, t20535: F, t2478: F, t38019: F, t544: F, t9287: F, t1429: F, t2365: F, t35888: F, t35893: F, t4391: F) -> (F, F, F, F, F) {
    let t46327 = F::cast_from(0.71500979903700853339e0_f64) * t41884;
    let t46331 = t20535 * t11549 * t2478;
    let t46335 = t544 * t38019 * t9287;
    let t46336 = F::cast_from(0.14896037479937677779e-1_f64) * t46335;
    let t46338 = t1429 * t2365 * t35888;
    let t46339 = F::cast_from(0.44688112439813033337e-1_f64) * t46338;
    let t46341 = t4391 * t2365 * t35893;
    (t46327, t46331, t46336, t46339, t46341)
}
