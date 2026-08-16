//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1444/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1444(t1: f64, t12635: f64, t12741: f64, t12860: f64, t1442: f64, t1507: f64, t15781: f64, t17921: f64, t18020: f64, t18031: f64, t18106: f64, t27552: f64, t3192: f64, t3234: f64, t3235: f64, t3244: f64, t3245: f64, t438: f64, t450: f64, t45773: f64, t45812: f64, t46390: f64, t5312: f64, t5413: f64, t54959: f64, t55734: f64, t55749: f64, t55751: f64, t55753: f64, t58941: f64, t59722: f64, t60141: f64, t9116: f64, t9117: f64) -> f64 {
    let t60235 = -0.1569271116311700736e4_f64 * t45773 * t5413 + 0.20408653907080965924e7_f64 * t9116 * t15781 * t9117 * t5312 + 0.1559479530529405812e2_f64 * t3234 * t3235 * t59722 - 0.12117441361606500412e2_f64 * t12635 * t18106 - 0.24951672488470492992e3_f64 * t12860 * t18020 - 0.23229342182245570105e2_f64 * t3192 * t450 * t58941 * t1 * t438 + 0.24234882723213000824e2_f64 * t12635 * t18031 + 0.22720202553012188272e1_f64 * t3244 * t3245 * t60141 + 0.15146801702008125515e1_f64 * t55734 + 0.15146801702008125515e1_f64 * t55749 - 0.99111859977581099115e3_f64 * t55751 - 0.49917948358154037253e1_f64 * t55753 - 0.10097867801338750343e1_f64 * t46390 + 0.18583473745796456084e3_f64 * t12741 * t45812 * t1442 * t1507 - 0.23967961564076583027e5_f64 * t27552 * t54959 * t17921;
    t60235
}
