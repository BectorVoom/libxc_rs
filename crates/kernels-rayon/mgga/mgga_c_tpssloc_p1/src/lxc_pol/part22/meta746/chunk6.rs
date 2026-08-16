//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2487/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2487(t1036: f64, t21483: f64, t1041: f64, t13969: f64, t21511: f64, t10413: f64, t10422: f64, t21531: f64, t10408: f64, t10937: f64, t13995: f64, t14511: f64, t17718: f64, t18021: f64, t21396: f64, t21520: f64, t21595: f64, t3070: f64, t3071: f64, t43361: f64, t48607: f64, t50148: f64, t50170: f64, t62602: f64, t69657: f64, t884: f64) -> (f64, f64) {
    let t70766 = t21483 * t1036;
    let t70792 = t1041 * t13969 * t21511;
    let t70800 = t10413 * t10422 * t21531;
    let t70802 = t10937 * t21520 / 144.0_f64 + t3070 * t3071 * t21595 * t884 / 4608.0_f64 + t13995 * t18021 / 1536.0_f64 - t14511 * t17718 / 1024.0_f64 - t50148 - 5.0_f64 / 768.0_f64 * t48607 * t10408 * t69657 + 5.0_f64 / 6912.0_f64 * t70792 - t50170 + t62602 / 1152.0_f64 - t43361 * t3071 * t21396 * t884 / 768.0_f64 - t70800 / 2304.0_f64;
    (t70766, t70802)
}
