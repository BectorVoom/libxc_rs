//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1312/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1312(t18464: f64, t5420: f64, t5424: f64, t13771: f64, t5728: f64, t13793: f64, t215: f64, t65595: f64, t13798: f64, t19469: f64, t19539: f64, t6259: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t69553 = t18464 * t5420;
    let t69555 = t18464 * t5424;
    let t69558 = t5728 * t13771;
    let t69561 = t65595 * t215 * t13793;
    let t69564 = t19469 * t215 * t13798;
    let t69654 = t6259 * t19539;
    (t69553, t69555, t69558, t69561, t69564, t69654)
}
