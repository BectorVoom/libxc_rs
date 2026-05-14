//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1118/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1118<F: Float>(t1354: F, t8894: F, t23772: F, t444: F, t3392: F, t23831: F, t94400: F, t128: F, t22708: F, t22711: F, t23855: F, t23700: F, t2001: F, t22632: F, t23732: F, t23733: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t94479 = t8894 * t1354;
    let t94507 = t23772 * t444;
    let t94508 = t3392 * t94507;
    let t94514 = t23831 * t94400;
    let t94518 = t128 * t22708 * t22711;
    let t94521 = t23855 * t22711;
    let t94524 = t3392 * t94400;
    let t94530 = t23831 * t23700;
    let t94535 = t2001 * t23700;
    let t94547 = t23732 * t22632 * t23733;
    (t94479, t94507, t94508, t94514, t94518, t94521, t94524, t94530, t94535, t94547)
}
