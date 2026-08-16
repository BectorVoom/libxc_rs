//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2490/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2490(t10422: f64, t21565: f64, t3070: f64, t10403: f64, t1041: f64, t10937: f64, t13995: f64, t14172: f64, t17998: f64, t21391: f64, t21566: f64, t3071: f64, t42388: f64, t43253: f64, t4347: f64, t4582: f64, t5873: f64, t62704: f64, t62766: f64, t62778: f64, t62780: f64, t70339: f64, t884: f64) -> f64 {
    let t70846 = t3070 * t10422 * t21565;
    let t70863 = -t43253 + t62704 / 384.0_f64 - 5.0_f64 / 768.0_f64 * t1041 * t4582 * t14172 * t70339 + t70846 / 2304.0_f64 - t10937 * t21566 / 288.0_f64 + t10403 * t3071 * t5873 * t4347 / 768.0_f64 + 5.0_f64 / 4608.0_f64 * t13995 * t17998 + 7.0_f64 / 648.0_f64 * t62766 - t62778 / 256.0_f64 + t62780 / 1152.0_f64 + t42388 * t3071 * t21391 * t884 / 768.0_f64;
    t70863
}
