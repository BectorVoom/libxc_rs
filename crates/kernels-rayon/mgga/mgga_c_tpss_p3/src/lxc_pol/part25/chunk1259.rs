//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1259/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1259(t5: f64, t21784: f64, t117: f64, t4525: f64, t6436: f64, t18934: f64, t18943: f64, t19466: f64, t19479: f64, t19491: f64, t21036: f64, t21038: f64, t21040: f64, t21042: f64, t21044: f64, t21046: f64, t21048: f64, t21050: f64) -> (f64, f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t21785 = piecewise3(t8, 0.0_f64, t21784);
    let t21786 = t21785 * t117;
    let t21790 = t6436 * t4525;
    let t21804 = t18934 + 7.0_f64 / 36.0_f64 * t19466 + t21036 / 8.0_f64 - t21038 / 24.0_f64 + t21040 / 384.0_f64 + 7.0_f64 / 576.0_f64 * t19479 + t21042 / 96.0_f64 - t21044 / 768.0_f64 - t21046 / 768.0_f64 + t18943 + 7.0_f64 / 144.0_f64 * t19491 + 5.0_f64 / 192.0_f64 * t21048 - t21050 / 192.0_f64;
    (t21785, t21786, t21790, t21804)
}
