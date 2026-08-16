//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1257/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1257(t40341: f64, t40345: f64, t37444: f64, t39069: f64, t39071: f64, t39072: f64, t40331: f64, t40334: f64, t41294: f64, t41296: f64, t41300: f64, t41305: f64, t41308: f64, t41311: f64, t41314: f64) -> f64 {
    let t42196 = 0.1440846329149835838e-2_f64 * t40341;
    let t42197 = 0.20496175532535769482e-3_f64 * t40345;
    let t42199 = -t41294 - t41296 + 0.162600798888400151e-2_f64 * t40331 - 0.39032073591371545778e-3_f64 * t40334 + t41300 + t39069 + t42196 - t42197 + t41305 + t41308 - t41311 + 0.12195059916630011325e-2_f64 * t37444 - t39071 - t39072 + t41314;
    t42199
}
