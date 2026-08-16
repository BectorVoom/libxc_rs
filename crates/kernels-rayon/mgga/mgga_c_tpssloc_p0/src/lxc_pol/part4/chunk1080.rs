//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 1080/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk1080(t248: f64, t3051: f64, t5681: f64, t1041: f64, t1616: f64, t4338: f64, t10408: f64, t1409: f64, t14219: f64, t14218: f64, t3071: f64, t2940: f64, t5804: f64) -> (f64, f64, f64, f64) {
    let t17906 = t248 * t3051 * t5681;
    let t17907 = t1041 * t17906;
    let t17919 = t1616 * t4338;
    let t17920 = t10408 * t17919;
    let t17923 = t14219 * t1409;
    let t17924 = t14218 * t17923;
    let t17925 = t3071 * t17924;
    let t17929 = 0.11696447245269292414e1_f64 * t2940 * t5804;
    (t17907, t17920, t17925, t17929)
}
