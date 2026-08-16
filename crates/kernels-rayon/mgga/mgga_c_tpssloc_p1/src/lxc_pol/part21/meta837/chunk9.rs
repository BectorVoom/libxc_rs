//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2987/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2987(t17906: f64, t3048: f64, t1041: f64, t248: f64, t43338: f64, t5677: f64, t1022: f64, t10403: f64, t10408: f64, t10413: f64, t10937: f64, t10957: f64, t13532: f64, t13537: f64, t13542: f64, t14211: f64, t1616: f64, t17593: f64, t17923: f64, t18016: f64, t18025: f64, t18030: f64, t2775: f64, t2960: f64, t3070: f64, t3071: f64, t3123: f64, t3131: f64, t42397: f64, t42505: f64, t42541: f64, t4347: f64, t49616: f64, t50027: f64, t5900: f64, t62055: f64, t62059: f64, t62291: f64) -> f64 {
    let t62441 = t3048 * t17906;
    let t62445 = t1041 * t248 * t43338 * t5677;
    let t62475 = -t62055 * t62291 * t3131 * t2775 * t62059 / 288.0_f64 + t42541 * t18016 / 576.0_f64 + t18030 * t3123 / 3072.0_f64 - 19.0_f64 / 1296.0_f64 * t10957 * t5900 + t62441 / 324.0_f64 - 5.0_f64 / 62208.0_f64 * t62445 + t10937 * t18025 / 108.0_f64 + 5.0_f64 / 6912.0_f64 * t3070 * t10408 * t1616 * t13532 + 5.0_f64 / 2592.0_f64 * t3070 * t42397 * t1616 * t13537 - t3070 * t3071 * t1616 * t13542 / 576.0_f64 - t10413 * t3071 * t49616 * t17923 / 2304.0_f64 + t10403 * t3071 * t14211 * t4347 * t1022 / 576.0_f64 - t42505 * t18016 / 108.0_f64 + t2960 * t17593 / 27.0_f64 - t50027 / 108.0_f64;
    t62475
}
