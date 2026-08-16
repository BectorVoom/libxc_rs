//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1741/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1741(t89947: f64, t89959: f64, t6573: f64, t6628: f64, t1774: f64, t22688: f64, t1042: f64, t1261: f64, t17202: f64, t17569: f64, t24612: f64, t24773: f64, t3711: f64, t5268: f64, t5293: f64, t5819: f64, t6587: f64, t69668: f64, t69700: f64, t82338: f64, t82351: f64, t82434: f64, t82441: f64, t88732: f64) -> (f64, f64, f64, f64) {
    let t89960 = t89947 + t89959;
    let t89978 = t6573 * t6628;
    let t90001 = t22688 * t1774;
    let t90012 = -0.45732285992607719436e-2_f64 * t5293 * t24773 - 0.51448821741683684366e-2_f64 * t1261 * t1042 * t17202 * t88732 + 0.34299214494455789577e-2_f64 * t17569 * t24612 + 0.17149607247227894789e-2_f64 * t3711 * t1042 * t5268 * t5819 * t6587 + 0.34299214494455789577e-2_f64 * t3711 * t1042 * t17202 * t90001 - 0.91464571985215438872e-2_f64 * t82338 + 0.22866142996303859718e-2_f64 * t82351 - 0.28582678745379824648e-3_f64 * t69668 - 0.57165357490759649296e-3_f64 * t69700 + 0.91464571985215438872e-2_f64 * t82434 + 0.57927562257303111285e-1_f64 * t82441;
    (t89960, t89978, t90001, t90012)
}
