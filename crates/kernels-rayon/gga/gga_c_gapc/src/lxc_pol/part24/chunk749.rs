//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 749/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk749(t1030: f64, t8906: f64, t4979: f64, t1631: f64, t190: f64, t3707: f64, t1743: f64, t3113: f64, t3112: f64, t3117: f64, t3123: f64, t8798: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8907 = t1030 * t8906;
    let t8908 = t8907 * t4979;
    let t8910 = t1631 * t190;
    let t8911 = t8910 * t3707;
    let t8912 = t1743 * t8911;
    let t8913 = t8912 * t4979;
    let t8915 = t3113 * t3707;
    let t8916 = t3112 * t8915;
    let t8917 = t8916 * t3117;
    let t8919 = t8798 * t3123;
    (t8908, t8910, t8911, t8913, t8915, t8916, t8917, t8919)
}
