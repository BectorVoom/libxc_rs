//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1687/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1687(t25304: f64, t7283: f64, t25949: f64, t786: f64, t1426: f64, t3999: f64, t25821: f64, t2106: f64, t530: f64, t6977: f64, t7348: f64, t1923: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t26069 = t25304 * t7283;
    let t26072 = t786 * t25949;
    let t26079 = t1426 * t3999;
    let t26148 = 22.0_f64 / 9.0_f64 * t25821;
    let t26161 = t530 * t2106;
    let t26169 = t7348 * t6977;
    let t26170 = t1923 * t26169;
    (t26069, t26072, t26079, t26148, t26161, t26169, t26170)
}
