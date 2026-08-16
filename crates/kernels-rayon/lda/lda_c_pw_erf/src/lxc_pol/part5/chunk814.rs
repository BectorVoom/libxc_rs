//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 814/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk814(t7433: f64, t7455: f64, t582: f64, t186: f64, t211: f64, t2443: f64, t808: f64, t2528: f64, t822: f64, t2405: f64, t793: f64, t184: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7456 = t7433 + t7455;
    let t7457 = t582 * t7456;
    let t7458 = t186 * t7457;
    let t7460 = 2.0_f64 / 15.0_f64 * t211 * t7458;
    let t7462 = 2.0_f64 / 5.0_f64 * t2443 * t808;
    let t7464 = 2.0_f64 / 5.0_f64 * t822 * t2528;
    let t7465 = t2405 * t793;
    let t7466 = t7465 * t184;
    (t7456, t7457, t7458, t7460, t7462, t7464, t7465, t7466)
}
