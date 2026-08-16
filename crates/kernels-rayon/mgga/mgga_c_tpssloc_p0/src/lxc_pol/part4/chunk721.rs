//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 721/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk721(t5045: f64, t974: f64, t1174: f64, t1198: f64, t1213: f64, t1218: f64, t1227: f64, t1232: f64, t1748: f64, t3490: f64, t3524: f64, t3542: f64, t3543: f64, t3547: f64, t3549: f64, t3573: f64, t4889: f64, t5014: f64, t5019: f64, t5024: f64, t5030: f64, t5033: f64, t5036: f64, t5041: f64) -> (f64, f64) {
    let t5046 = t974 * t5045;
    let t5051 = t1213 * t5014 / 3072.0_f64 - t5019 * t1218 / 576.0_f64 + t5024 * t1232 / 864.0_f64 - t3490 * t1748 / 4608.0_f64 - t1227 * t5030 / 4608.0_f64 + t1174 * t5033 / 216.0_f64 - t5036 / 108.0_f64 - t3524 / 6912.0_f64 + t3573 / 4608.0_f64 - t5041 / 864.0_f64 + t4889 * t1198 / 108.0_f64 - t1174 * t5046 / 288.0_f64 - t3549 / 864.0_f64 - t3542 + t3543 / 4608.0_f64 - t3547;
    (t5046, t5051)
}
