//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2050/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2050(t241: f64, t820: f64, t94491: f64, t5697: f64, t94429: f64, t5701: f64, t27928: f64, t9775: f64, t13775: f64, t25986: f64, t2661: f64, t25978: f64, t5614: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t98115 = t820 * t94491 * t241;
    let t98128 = t94429 * t5697;
    let t98129 = 0.16006300097412701803e-1_f64 * t98128;
    let t98130 = t94429 * t5701;
    let t98131 = 0.40015750243531754508e-2_f64 * t98130;
    let t98141 = t9775 * t27928;
    let t98144 = t2661 * t25986 * t13775;
    let t98145 = 0.28582678745379824648e-4_f64 * t98144;
    let t98146 = t25978 * t5614;
    (t98115, t98129, t98131, t98141, t98145, t98146)
}
