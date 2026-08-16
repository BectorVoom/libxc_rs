//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1213/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1213(t5: f64, t129196: f64, t129243: f64, t117: f64, t32855: f64, t4248: f64, t27123: f64, t8749: f64, t27126: f64, t32866: f64, t7732: f64, t1310: f64, t25805: f64, t28025: f64, t28030: f64, t29444: f64, t29459: f64, t32825: f64, t34419: f64, t4297: f64, t508: f64, t6985: f64, t7591: f64, t8158: f64) -> (f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t129245 = piecewise3(t8, 0.0_f64, t129196 + t129243);
    let t129246 = t129245 * t117;
    let t129251 = t4248 * t32855;
    let t129253 = t27123 * t8749;
    let t129255 = t27126 * t8749;
    let t129257 = t7732 * t32866;
    let t129265 = -t129246 * t508 - t1310 * t34419 - 2.0_f64 * t25805 * t8158 - 2.0_f64 * t28025 * t8158 - 2.0_f64 * t28030 * t7591 - 2.0_f64 * t29444 * t6985 - 2.0_f64 * t29459 * t6985 - 2.0_f64 * t32825 * t4297 - 2.0_f64 * t129251 - 2.0_f64 * t129253 - 2.0_f64 * t129255 - 2.0_f64 * t129257;
    (t129246, t129265)
}
