//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3187/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3187(t15503: f64, t15640: f64, t19025: f64, t3535: f64, t1202: f64, t19032: f64, t15498: f64, t4993: f64, t15486: f64, t5024: f64, t1090: f64, t11668: f64, t11678: f64, t1218: f64, t1227: f64, t1232: f64, t15654: f64, t15708: f64, t18205: f64, t18941: f64, t3243: f64, t3447: f64, t3494: f64, t3577: f64, t3578: f64, t45128: f64, t4582: f64, t4729: f64, t4987: f64, t5012: f64, t52935: f64, t52942: f64, t53249: f64, t55716: f64, t5971: f64, t61798: f64, t61910: f64, t6225: f64) -> f64 {
    let t66120 = t15503 * t15640;
    let t66147 = t3535 * t19025;
    let t66150 = t1202 * t19032;
    let t66153 = t15498 * t4993;
    let t66155 = t5024 * t15486;
    let t66157 = -t3577 * t3578 * t5012 * t4729 / 576.0_f64 - 5.0_f64 / 2592.0_f64 * t3577 * t45128 * t18205 * t15708 - t66120 / 108.0_f64 + 5.0_f64 / 6912.0_f64 * t1227 * t4582 * t4987 * t61798 + 5.0_f64 / 2304.0_f64 * t1227 * t4582 * t15654 * t61910 - t52935 / 432.0_f64 - 7.0_f64 / 162.0_f64 * t3447 * t53249 * t55716 - t3577 * t3578 * t18941 * t1090 / 2304.0_f64 + 5.0_f64 / 6912.0_f64 * t11678 * t11668 * t6225 * t3243 + 5.0_f64 / 13824.0_f64 * t3577 * t11668 * t5971 * t3494 - t52942 / 1728.0_f64 + 19.0_f64 / 864.0_f64 * t66147 * t1218 - 19.0_f64 / 1296.0_f64 * t66150 * t1232 + t66153 / 324.0_f64 + t66155 / 324.0_f64;
    t66157
}
