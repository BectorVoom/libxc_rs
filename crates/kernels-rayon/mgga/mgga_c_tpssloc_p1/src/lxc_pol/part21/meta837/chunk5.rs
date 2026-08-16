//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2983/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2983(t10413: f64, t10422: f64, t17700: f64, t1023: f64, t10403: f64, t10408: f64, t13611: f64, t1616: f64, t2771: f64, t2780: f64, t3039: f64, t3070: f64, t3071: f64, t42397: f64, t42735: f64, t42752: f64, t4582: f64, t4600: f64, t48607: f64, t49743: f64, t49852: f64, t49871: f64, t49873: f64, t49877: f64, t49884: f64, t49887: f64, t5873: f64, t61524: f64, t62091: f64) -> f64 {
    let t62306 = t10413 * t10422 * t17700;
    let t62333 = -5.0_f64 / 15552.0_f64 * t49852 + t42735 / 13824.0_f64 + t42752 / 7776.0_f64 - t49871 / 5184.0_f64 - t62306 / 3456.0_f64 + 5.0_f64 / 1296.0_f64 * t48607 * t42397 * t61524 + t10403 * t3071 * t5873 * t2780 / 2304.0_f64 + 5.0_f64 / 6912.0_f64 * t10403 * t10408 * t5873 * t2771 + t3070 * t3071 * t1616 * t13611 / 2304.0_f64 + t49743 * t4600 / 144.0_f64 - t49873 / 864.0_f64 - t49877 / 324.0_f64 - t3039 * t4582 * t62091 * t1023 / 1536.0_f64 - t49884 / 2304.0_f64 - t49887 / 384.0_f64;
    t62333
}
