//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1412/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1412(t15174: f64, t5186: f64, t8686: f64, t1102: f64, t26224: f64, t58311: f64, t8700: f64, t14871: f64, t15374: f64, t17529: f64, t4305: f64, t3061: f64, t8697: f64) -> (f64, f64, f64, f64, f64) {
    let t59205 = 0.3103500882342370105e4_f64 * t8686 * t15174 * t5186;
    let t59209 = 0.12304676425209353917e5_f64 * t1102 * t26224 * t58311 * t8700;
    let t59212 = 0.62336721237753107879e3_f64 * t1102 * t14871 * t15374;
    let t59214 = 0.14035736153892489771e2_f64 * t4305 * t17529;
    let t59218 = 0.6233672123775310788e3_f64 * t1102 * t8697 * t58311 * t3061;
    (t59205, t59209, t59212, t59214, t59218)
}
