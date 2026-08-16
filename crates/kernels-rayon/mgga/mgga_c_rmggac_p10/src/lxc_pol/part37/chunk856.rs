//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 856/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk856(t2842: f64, t69205: f64, t3046: f64, t30526: f64, t556: f64, t13902: f64, t1612: f64, t11704: f64, t13905: f64, t1587: f64, t1326: f64, t13911: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t75364 = t69205 * t2842;
    let t75367 = t30526 * t3046 * t556;
    let t75369 = t13902 * t1612;
    let t75371 = t13905 * t11704;
    let t75373 = t3046 * t1587;
    let t75374 = t1326 * t75373;
    let t75375 = t13911 * t75374;
    (t75364, t75367, t75369, t75371, t75373, t75374, t75375)
}
