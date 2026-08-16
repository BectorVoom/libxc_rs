//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 880/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk880(t1096: f64, t3325: f64, t3269: f64, t3075: f64, t1079: f64, t1071: f64, t3057: f64, t3259: f64, t994: f64, t342: f64, t992: f64, t338: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11177 = t1096 * t3325;
    let t11178 = t3269 * t11177;
    let t11183 = t3075 * t1096;
    let t11184 = t1079 * t11183;
    let t11187 = t3057 * t1071;
    let t11190 = t994 * t3259;
    let t11195 = t342 * t3259;
    let t11198 = t992 * t992;
    let t11199 = 1.0_f64 / t11198;
    let t11200 = t338 * t11199;
    (t11178, t11184, t11187, t11190, t11195, t11200)
}
