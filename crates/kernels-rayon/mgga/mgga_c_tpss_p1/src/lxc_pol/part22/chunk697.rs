//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 697/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk697(t3357: f64, t1254: f64, t219: f64, t1257: f64, t536: f64, t73: f64, t1265: f64, param_beta: f64) -> (f64, f64, f64, f64, f64) {
    let t3358 = param_beta * t3357;
    let t3360 = t1254 * t219;
    let t3364 = 1.0_f64 / t1257 / t536;
    let t3365 = t73 * t3364;
    let t3366 = t1265 * t1265;
    (t3358, t3360, t3364, t3365, t3366)
}
