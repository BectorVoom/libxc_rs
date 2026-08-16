//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1469/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1469(t1174: f64, t1214: f64, t1227: f64, t1230: f64, t15740: f64, t22149: f64, t22154: f64, t22218: f64, t22301: f64, t248: f64, t3440: f64, t3508: f64, t45037: f64, t4889: f64, t5024: f64, t52836: f64, t66057: f64, t72703: f64, t72705: f64, t72708: f64, t72727: f64, t72733: f64, t72798: f64, t77981: f64, t78031: f64, t79018: f64) -> f64 {
    let t79188 = 7.0_f64 / 1536.0_f64 * t45037 * t248 * t1214 * t79018 * t3508 + t52836 * t22301 / 768.0_f64 + t5024 * t22218 / 36.0_f64 - t1227 * t248 * t1230 * t77981 / 4608.0_f64 + t72703 / 27.0_f64 + 2.0_f64 / 27.0_f64 * t72705 + t72708 / 27.0_f64 - t66057 / 162.0_f64 - t15740 * t22154 / 384.0_f64 - 4.0_f64 / 27.0_f64 * t4889 * t22149 + t1174 * t3440 * t78031 / 54.0_f64 - t72727 / 288.0_f64 - 209.0_f64 / 972.0_f64 * t72733 + 19.0_f64 / 216.0_f64 * t72798;
    t79188
}
