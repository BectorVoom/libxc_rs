//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1122/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1122(t112: f64, t843: f64, t239: f64, t655: f64, t2339: f64, t624: f64, t10208: f64, t68: f64, t1923: f64, t1927: f64, t72: f64, t2247: f64, t38: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t94973 = t843 * t112;
    let t94975 = t239 * t655;
    let t94978 = t624 * t2339;
    let t94982 = t68 * t10208;
    let t95253 = 1232.0_f64 / 81.0_f64 * t1923 * t843 * t72 * t1927;
    let t95293 = t2247 * t38 * t239;
    (t94973, t94975, t94978, t94982, t95253, t95293)
}
