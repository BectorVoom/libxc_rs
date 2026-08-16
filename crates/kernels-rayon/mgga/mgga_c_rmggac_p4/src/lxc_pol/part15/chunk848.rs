//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 848/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk848(t275: f64, t9031: f64, t118: f64, t2281: f64, t498: f64, t7418: f64, t7244: f64, t9153: f64, t8876: f64, t942: f64, t4961: f64, t668: f64) -> (f64, f64, f64, f64, f64) {
    let t41905 = 2.0_f64 * t275 * t9031;
    let t41914 = t7418 * t118 * t2281 * t498;
    let t41922 = t7244 * t9153;
    let t41929 = 0.4726e1_f64 * t942 * t8876;
    let t41932 = t4961 * t668;
    (t41905, t41914, t41922, t41929, t41932)
}
