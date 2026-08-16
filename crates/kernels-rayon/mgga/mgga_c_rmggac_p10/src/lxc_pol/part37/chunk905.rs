//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 905/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk905(t2079: f64, t262: f64, t36: f64, t8794: f64, t14302: f64, t75374: f64, t14305: f64, t75416: f64, t1326: f64, t14309: f64, t1652: f64, t3046: f64) -> (f64, f64, f64, f64) {
    let t76242 = t2079 * t262 * t36 * t8794;
    let t76244 = t14302 * t75374;
    let t76246 = t14305 * t75416;
    let t76250 = t14309 * t1326 * t3046 * t1652;
    (t76242, t76244, t76246, t76250)
}
