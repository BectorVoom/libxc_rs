//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 808/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk808(t26021: f64, t7262: f64, t820: f64, t843: f64, t1401: f64, t241: f64, t3920: f64, t7246: f64, t2023: f64, t2453: f64, t3908: f64, t72: f64, t7307: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t26022 = 0.90357964994909313586e-5_f64 * t26021;
    let t26024 = t820 * t7262 * t843;
    let t26025 = t26024 * t1401;
    let t26028 = t820 * t7262 * t241;
    let t26040 = 0.13009920719177044025e-1_f64 * t7246 * t3920;
    let t26041 = t2453 * t2023;
    let t26043 = 0.11565819519348392139e-2_f64 * t26041 * t3908;
    let t26049 = t7307 * t72;
    (t26022, t26024, t26025, t26028, t26040, t26043, t26049)
}
