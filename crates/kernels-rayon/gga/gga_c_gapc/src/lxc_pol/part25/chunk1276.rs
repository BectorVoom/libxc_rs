//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1276/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1276(t11326: f64, t8885: f64, t1882: f64, t20461: f64, t21249: f64, t5462: f64, t674: f64, t11463: f64, t505: f64, t5713: f64, t9066: f64, t116: f64, t33257: f64) -> (f64, f64, f64, f64) {
    let t35162 = t11326 * t8885;
    let t35169 = t5462 * t1882 * t20461 * t674 * t21249;
    let t35173 = t11463 * t9066 * t505 * t5713;
    let t35175 = t116 * t33257;
    (t35162, t35169, t35173, t35175)
}
