//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1157/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1157(t1206: f64, t3189: f64, t9283: f64, t14605: f64, t1115: f64, t13810: f64, t14188: f64, t14327: f64, t14479: f64, t14585: f64, t14589: f64, t14593: f64, t14597: f64, t14603: f64, t14883: f64, t14888: f64, t3066: f64, t3207: f64, t6793: f64, t8793: f64) -> (f64, f64, f64) {
    let t14894 = t1206 * t3189;
    let t14895 = t9283 * t14894;
    let t14898 = 7.0_f64 / 2304.0_f64 * t14605;
    let t14899 = -t13810 + t14479 / 48.0_f64 - t1115 * t14327 / 96.0_f64 - t14585 / 768.0_f64 - t14589 / 768.0_f64 - t14593 / 192.0_f64 - t14597 / 768.0_f64 - t3066 * t14883 / 16.0_f64 + t6793 * t14888 / 48.0_f64 + t8793 * t14188 / 48.0_f64 + t14603 / 384.0_f64 - t3207 * t14895 / 16.0_f64 + t14898;
    (t14894, t14895, t14899)
}
