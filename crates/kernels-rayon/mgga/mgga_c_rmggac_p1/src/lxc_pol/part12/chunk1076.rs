//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 1076/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk1076(t27111: f64, t3351: f64, t515: f64, t9188: f64, t16156: f64, t9184: f64, t1001: f64, t570: f64, t9210: f64, t26283: f64, t26291: f64, t29838: f64, t36992: f64, t36994: f64, t36998: f64, t37000: f64, t37006: f64, t41440: f64, t41443: f64, t41518: f64, t42196: f64, t42199: f64, t42201: f64, t42205: f64, t42207: f64, t42211: f64) -> f64 {
    let t42215 = t3351 * t9188 * t515 * t27111;
    let t42217 = t16156 * t9184;
    let t42222 = t3351 * t9210 * t515 * t570 * t1001;
    let t42227 = -0.14369090042236570277e1_f64 * t26283 * t41440 - 0.71845450211182851384e0_f64 * t26291 * t41443 + 0.95793933614910468512e0_f64 * t29838 * t41518 - 0.13334279030964389289e0_f64 * t42196 + 2.0_f64 * t36992 - 0.6818665413561335432e-1_f64 * t42199 - 0.72732431077987577943e-1_f64 * t42201 - 0.4726e1_f64 * t36994 + t42205 - t42207 + 0.25538759935978703638e-4_f64 * t42211 - 0.25538759935978703638e-4_f64 * t42215 + 0.59590439850616975156e-4_f64 * t42217 - 0.85129199786595678796e-5_f64 * t42222 + 0.79828278012425390426e-1_f64 * t36998 - 0.39914139006212695213e-1_f64 * t37000 + 0.47896966807455234256e0_f64 * t37006;
    t42227
}
