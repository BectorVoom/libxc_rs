//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1984/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1984(t22227: f64, t22242: f64, t475: f64, t1214: f64, t248: f64, t21510: f64, t4972: f64, t4582: f64, t11834: f64, t1213: f64, t1227: f64, t15717: f64, t15719: f64, t15727: f64, t15731: f64, t15735: f64, t1737: f64, t1748: f64, t18978: f64, t18980: f64, t18987: f64, t19026: f64, t19033: f64, t19041: f64, t19080: f64, t22208: f64, t22214: f64, t22218: f64, t5024: f64, t6203: f64, t6211: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22243 = t22227 + t22242;
    let t22244 = t22243 * t475;
    let t22246 = t248 * t1214 * t22244;
    let t22257 = t4972 * t21510;
    let t22258 = t4582 * t22257;
    let t22267 = -t18978 / 144.0_f64 - t18980 / 1152.0_f64 + t18987 / 216.0_f64 - 5.0_f64 / 5184.0_f64 * t1227 * t22208 + t5024 * t6211 / 144.0_f64 - t1227 * t22214 / 4608.0_f64 - t1227 * t22218 / 768.0_f64 + t11834 + t1213 * t22246 / 3072.0_f64 + t15717 / 864.0_f64 - t15719 / 4608.0_f64 + t15727 / 54.0_f64 - t15731 / 4608.0_f64 + t15735 / 6912.0_f64 - t19041 / 2304.0_f64 - 5.0_f64 / 864.0_f64 * t5024 * t6203 - t1227 * t22258 / 768.0_f64 + 19.0_f64 / 576.0_f64 * t19026 * t1737 - 19.0_f64 / 864.0_f64 * t19033 * t1748 - t19080 * t1737 / 96.0_f64;
    (t22243, t22244, t22246, t22257, t22258, t22267)
}
