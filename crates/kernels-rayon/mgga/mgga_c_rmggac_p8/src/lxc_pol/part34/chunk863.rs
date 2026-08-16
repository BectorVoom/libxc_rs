//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 863/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk863(t13798: f64, t40138: f64, t13802: f64, t61965: f64, t14131: f64, t68422: f64, t8421: f64, t21714: f64, t8426: f64, t14125: f64, t68622: f64, t8416: f64) -> (f64, f64, f64, f64, f64) {
    let t75450 = t40138 * t13798;
    let t75452 = t61965 * t13802;
    let t75455 = t14131 * t68422 * t8421;
    let t75458 = t14131 * t21714 * t8426;
    let t75461 = t68622 * t14125 * t8416;
    (t75450, t75452, t75455, t75458, t75461)
}
