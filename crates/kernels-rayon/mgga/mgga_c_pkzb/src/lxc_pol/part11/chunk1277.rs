//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1277/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1277(t22500: f64, t9856: f64, t11323: f64, t2328: f64, t11327: f64, t10176: f64, t3147: f64, t3841: f64, t8028: f64, t10170: f64, t10183: f64, t10009: f64, t11181: f64, t22561: f64, t22567: f64, t22575: f64, t22699: f64, t22750: f64, t31052: f64, t31092: f64, t31094: f64, t3135: f64, t3819: f64, t3823: f64, t6282: f64, t6323: f64, t8071: f64, t8107: f64, t8177: f64, t889: f64, t9881: f64, t9981: f64, t9986: f64, t9989: f64, t9993: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t31109 = 18.0_f64 * t22500 * t9856;
    let t31111 = 0.10254018858216406658e4_f64 * t2328 * t11323;
    let t31113 = 0.10389515463408878255e3_f64 * t2328 * t11327;
    let t31115 = 0.35089341735807877242e1_f64 * t3147 * t10176;
    let t31117 = 0.51947577317044391276e2_f64 * t8028 * t3841;
    let t31122 = 0.30762056574649219972e4_f64 * t3147 * t10170;
    let t31124 = 0.51947577317044391276e2_f64 * t3147 * t10183;
    let t31151 = 18.0_f64 * t22575 * t10009 - t31052 + 0.10526802520742363173e2_f64 * t22699 * t9981 + t31092 + t31094 - t31109 - 0.35089341735807877242e1_f64 * t8071 * t9981 + 0.51947577317044391276e2_f64 * t8107 * t9986 - 0.31168546390226634765e3_f64 * t22567 * t9881 + 0.10389515463408878255e3_f64 * t8107 * t9989 + 0.30762056574649219972e4_f64 * t22750 * t9993 + 0.6233709278045326953e3_f64 * t6282 * t11181 * t889 - 0.31168546390226634765e3_f64 * t6323 * t3823 * t3135 + 0.30762056574649219973e4_f64 * t6282 * t3819 * t8177 * t889 - 0.31168546390226634766e3_f64 * t22561 * t9986;
    (t31109, t31111, t31113, t31115, t31117, t31122, t31124, t31151)
}
