//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 747/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk747<F: Float>(t1587: F, t664: F, t2067: F, t26: F, t25525: F, t2079: F, t262: F, t570: F, t830: F, t551: F, t2068: F, t558: F, t2073: F, t1614: F, t265: F, t1652: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t40983 = t664 * t1587;
    let t40998 = t2067 * t26;
    let t40999 = t25525 * t40998;
    let t41021 = t2079 * t262 * t830 * t570;
    let t41027 = t830 * t551;
    let t41028 = t262 * t41027;
    let t41029 = t2068 * t41028;
    let t41031 = t830 * t558;
    let t41032 = t262 * t41031;
    let t41033 = t2073 * t41032;
    let t41035 = t265 * t1614;
    let t41036 = t262 * t41035;
    let t41037 = t2073 * t41036;
    let t41041 = t2079 * t262 * t265 * t1652;
    (t40983, t40998, t40999, t41021, t41027, t41028, t41029, t41031, t41032, t41033, t41035, t41036, t41037, t41041)
}
