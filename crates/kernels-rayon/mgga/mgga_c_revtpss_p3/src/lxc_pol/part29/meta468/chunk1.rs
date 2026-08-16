//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1726/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1726(t25223: f64, t25225: f64, t25229: f64, t25235: f64, t25238: f64, t25246: f64, t25248: f64, t26450: f64, t26454: f64, t26457: f64, t26472: f64) -> f64 {
    let t26473 = t26450 + 0.32012600194825403606e-1_f64 * t25223 - 0.34299214494455789578e-2_f64 * t25225 + 0.57165357490759649296e-4_f64 * t25229 - t26454 - 0.4065600224742826258e-3_f64 * t25235 + t25238 / 8.0_f64 + t26457 - 0.10164000561857065645e-3_f64 * t25246 + 0.17149607247227894789e-1_f64 * t25248 + t26472;
    t26473
}
