//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 1214/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk1214(t15977: f64, t17: f64, t2663: f64, t5157: f64, t12103: f64, t12105: f64, t12109: f64, t12114: f64, t12116: f64, t12118: f64, t12123: f64, t12477: f64, t15970: f64, t15972: f64, t15973: f64, t15974: f64, t15975: f64, t15976: f64, t1799: f64, t3719: f64, t3918: f64, t5122: f64, t9797: f64, t9820: f64, t9824: f64) -> (f64, f64, f64) {
    let t15978 = t17 * t15977;
    let t15979 = t5157 * t2663;
    let t15980 = 0.24415263074675393405e-3_f64 * t15979;
    let t15981 = -3.0_f64 * t12477 * t1799 * t3918 + 3.0_f64 * t3719 * t3918 * t5122 + t12103 - t12105 - t12109 - t12114 + t12116 + t12118 + t12123 + t15970 + t15972 + t15973 - t15974 + t15975 + t15976 + t15978 + t15980 + t9797 - t9820 - t9824;
    (t15978, t15980, t15981)
}
