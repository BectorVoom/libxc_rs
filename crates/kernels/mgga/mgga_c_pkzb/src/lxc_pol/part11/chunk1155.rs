//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1155/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1155<F: Float>(t11323: F, t2328: F, t11327: F, t10176: F, t3147: F, t3841: F, t8028: F, t10170: F, t10183: F, t10009: F, t11181: F, t22561: F, t22567: F, t22575: F, t22699: F, t22750: F, t31052: F, t31092: F, t31094: F, t31109: F, t3135: F, t3819: F, t3823: F, t6282: F, t6323: F, t8071: F, t8107: F, t8177: F, t889: F, t9881: F, t9981: F, t9986: F, t9989: F, t9993: F) -> (F, F, F, F, F, F, F) {
    let t31111 = 0.10254018858216406658e4 * t2328 * t11323;
    let t31113 = 0.10389515463408878255e3 * t2328 * t11327;
    let t31115 = 0.35089341735807877242e1 * t3147 * t10176;
    let t31117 = 0.51947577317044391276e2 * t8028 * t3841;
    let t31122 = 0.30762056574649219972e4 * t3147 * t10170;
    let t31124 = 0.51947577317044391276e2 * t3147 * t10183;
    let t31151 = 18.0 * t22575 * t10009 - t31052 + 0.10526802520742363173e2 * t22699 * t9981 + t31092 + t31094 - t31109 - 0.35089341735807877242e1 * t8071 * t9981 + 0.51947577317044391276e2 * t8107 * t9986 - 0.31168546390226634765e3 * t22567 * t9881 + 0.10389515463408878255e3 * t8107 * t9989 + 0.30762056574649219972e4 * t22750 * t9993 + 0.6233709278045326953e3 * t6282 * t11181 * t889 - 0.31168546390226634765e3 * t6323 * t3823 * t3135 + 0.30762056574649219973e4 * t6282 * t3819 * t8177 * t889 - 0.31168546390226634766e3 * t22561 * t9986;
    (t31111, t31113, t31115, t31117, t31122, t31124, t31151)
}
