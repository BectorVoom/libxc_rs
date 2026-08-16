//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2976/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2976(t10482: f64, t5872: f64, t10413: f64, t10422: f64, t17924: f64, t17959: f64, t376: f64, t10480: f64, t13969: f64, t17672: f64, t10408: f64, t1041: f64, t14164: f64, t14207: f64, t14213: f64, t14228: f64, t14234: f64, t17151: f64, t17177: f64, t17182: f64, t17673: f64, t17925: f64, t2770: f64, t3070: f64, t3071: f64, t3130: f64, t3131: f64, t42388: f64, t42397: f64, t42508: f64, t43322: f64, t4582: f64, t4594: f64, t4652: f64, t49702: f64, t62044: f64, t62049: f64, t62055: f64, t62057: f64, t62059: f64, t62064: f64) -> (f64, f64) {
    let t62079 = t5872 * t10482;
    let t62085 = t10413 * t10422 * t17924;
    let t62091 = t376 * t17959;
    let t62099 = t10480 * t13969 * t17672;
    let t62101 = t1041 * t4582 * t14164 * t62044 / 768.0_f64 - t62049 / 216.0_f64 + t14207 * t4652 / 768.0_f64 + 5.0_f64 / 1728.0_f64 * t62055 * t62057 * t3131 * t2770 * t62059 - 5.0_f64 / 3456.0_f64 * t62064 * t62057 * t14234 + 5.0_f64 / 6912.0_f64 * t3070 * t10408 * t17177 * t14228 + 5.0_f64 / 2592.0_f64 * t3070 * t42397 * t17151 * t14228 + t42508 * t17925 / 216.0_f64 - t49702 / 1728.0_f64 + t42388 * t3071 * t62079 * t14213 / 384.0_f64 - t62085 / 1728.0_f64 - t3070 * t3071 * t17182 * t14228 / 1152.0_f64 + t3130 * t4582 * t62091 * t4594 / 768.0_f64 + t43322 * t17673 / 256.0_f64 + t62099 / 384.0_f64;
    (t62091, t62101)
}
