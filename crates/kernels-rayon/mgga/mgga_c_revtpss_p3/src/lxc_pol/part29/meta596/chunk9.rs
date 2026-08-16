//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2012/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2012(t98983: f64, t98991: f64, t99000: f64, t99006: f64, t92991: f64, t95671: f64, t98985: f64, t98989: f64, t98993: f64, t98995: f64, t98997: f64, t99002: f64) -> f64 {
    let t103273 = 0.4065600224742826258e-3_f64 * t98983;
    let t103276 = 0.80031500487063509014e-2_f64 * t98991;
    let t103280 = 0.22866142996303859718e-3_f64 * t99000;
    let t103283 = 0.57165357490759649296e-4_f64 * t99006;
    let t103284 = t103273 + 0.68598428988911579156e-2_f64 * t98985 - 0.51448821741683684367e-2_f64 * t98989 + t103276 + 0.34299214494455789578e-2_f64 * t98993 - t98995 / 24.0_f64 + 0.17149607247227894789e-1_f64 * t98997 - t103280 + 0.54208002996571016773e-3_f64 * t99002 - t95671 + 0.81312004494856525159e-4_f64 * t92991 + t103283;
    t103284
}
