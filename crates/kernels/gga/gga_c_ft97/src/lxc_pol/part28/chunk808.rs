//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 808/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk808<F: Float>(t11120: F, t45499: F, t3018: F, t62: F, t8417: F, t971: F, t1851: F, t3170: F, t110: F, t38477: F, t38463: F, t1587: F, t480: F, t370: F, t8216: F, t1780: F, t1852: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t45500 = t45499 * t11120;
    let t45572 = t3018 * t62;
    let t46565 = t971 * t8417;
    let t46727 = t3170 * t1851;
    let t46874 = t38477 * t110;
    let t46881 = t38463 * t110;
    let t47007 = t1587 * t480;
    let t47120 = t370 * t480;
    let t47273 = t8216 * t971;
    let t47399 = t1780 * t1852;
    (t45500, t45572, t46565, t46727, t46874, t46881, t47007, t47120, t47273, t47399)
}
