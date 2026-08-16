//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 832/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk832(t2079: f64, t262: f64, t570: f64, t830: f64, t551: f64, t2068: f64, t558: f64, t2073: f64, t1614: f64, t265: f64, t1652: f64, t1587: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
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
    let t41042 = 0.18183107769496894486e-1_f64 * t41041;
    let t41055 = t265 * t1587;
    (t41021, t41027, t41028, t41029, t41031, t41032, t41033, t41035, t41036, t41037, t41042, t41055)
}
