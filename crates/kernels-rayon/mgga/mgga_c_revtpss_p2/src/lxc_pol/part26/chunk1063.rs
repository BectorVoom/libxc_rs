//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1063/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1063(t26153: f64, t508: f64, t2106: f64, t530: f64, t25865: f64, t6977: f64, t7348: f64, t1923: f64, t2047: f64, t25146: f64, t10309: f64, t7342: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26154 = t508 * t26153;
    let t26161 = t530 * t2106;
    let t26162 = t26161 * t25865;
    let t26169 = t7348 * t6977;
    let t26170 = t1923 * t26169;
    let t26172 = t2047 * t25146;
    let t26175 = t10309 * t7342;
    (t26154, t26162, t26169, t26170, t26172, t26175)
}
