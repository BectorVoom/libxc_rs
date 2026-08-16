//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1373/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1373(t2435: f64, t5760: f64, t545: f64, t5710: f64, t869: f64, t689: f64, t225: f64, t9990: f64, t213: f64, t2777: f64, t5759: f64, t2439: f64) -> (f64, f64, f64, f64) {
    let t14166 = t2435 * t5760;
    let t14188 = t545 * t5710;
    let t14189 = t869 * t14188;
    let t14191 = 0.10975748638225852664e-1_f64 * t689 * t14189;
    let t14192 = t225 * t9990;
    let t14193 = t213 * t14192;
    let t14202 = t2777 * t5759;
    let t14203 = t2439 * t14202;
    (t14166, t14191, t14193, t14203)
}
