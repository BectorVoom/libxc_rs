//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 960/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk960<F: Float>(t2109: F, t7780: F, t1980: F, t31028: F, t7476: F, t1988: F, t7701: F, t7705: F, t1982: F, t2015: F, t7452: F, t7440: F, t7444: F) -> (F, F, F, F, F, F, F) {
    let t31752 = t7780 * t2109;
    let t31759 = t1980 * t7476 * t31028;
    let t31761 = t1988 * t7701;
    let t31763 = t1988 * t7705;
    let t31773 = t2015 * t1982;
    let t31774 = t31773 * t7452;
    let t31782 = t7440 * t7444;
    (t31752, t31759, t31761, t31763, t31773, t31774, t31782)
}
