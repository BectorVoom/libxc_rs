//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 820/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk820(t1327: f64, t142: f64, t8888: f64, t599: f64, t8406: f64, t1181: f64, t7346: f64, t7678: f64, t7686: f64, t7697: f64, t7698: f64, t7710: f64, t7713: f64, t7718: f64, t7721: f64, t7726: f64, t8876: f64, t8879: f64, t8882: f64, t8885: f64) -> (f64, f64, f64, f64) {
    let t8889 = t142 * t1327;
    let t8890 = t8888 * t8889;
    let t8896 = t599 * t8406;
    let t8897 = t1181 * t8896;
    let t8898 = t7346 * t8897;
    let t8900 = -t8876 / 64.0_f64 - t8879 / 192.0_f64 + t7678 + 0.20007875121765877254e-2_f64 * t7686 - t7697 - 0.28015625e-1_f64 * t8882 + t8885 / 48.0_f64 + t8890 / 48.0_f64 - 0.21437009059034868486e-3_f64 * t7698 + 0.31448092289604152067e-3_f64 * t7710 - 0.42874018118069736972e-3_f64 * t7713 - t7718 - 0.10718504529517434243e-3_f64 * t7721 - t7726 + 0.10718504529517434243e-3_f64 * t8898;
    (t8889, t8896, t8897, t8900)
}
