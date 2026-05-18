//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1444/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1444<F: Float>(t1: F, t12635: F, t12741: F, t12860: F, t1442: F, t1507: F, t15781: F, t17921: F, t18020: F, t18031: F, t18106: F, t27552: F, t3192: F, t3234: F, t3235: F, t3244: F, t3245: F, t438: F, t450: F, t45773: F, t45812: F, t46390: F, t5312: F, t5413: F, t54959: F, t55734: F, t55749: F, t55751: F, t55753: F, t58941: F, t59722: F, t60141: F, t9116: F, t9117: F) -> F {
    let t60235 = -F::new(0.1569271116311700736e4) * t45773 * t5413 + F::new(0.20408653907080965924e7) * t9116 * t15781 * t9117 * t5312 + F::new(0.1559479530529405812e2) * t3234 * t3235 * t59722 - F::new(0.12117441361606500412e2) * t12635 * t18106 - F::new(0.24951672488470492992e3) * t12860 * t18020 - F::new(0.23229342182245570105e2) * t3192 * t450 * t58941 * t1 * t438 + F::new(0.24234882723213000824e2) * t12635 * t18031 + F::new(0.22720202553012188272e1) * t3244 * t3245 * t60141 + F::new(0.15146801702008125515e1) * t55734 + F::new(0.15146801702008125515e1) * t55749 - F::new(0.99111859977581099115e3) * t55751 - F::new(0.49917948358154037253e1) * t55753 - F::new(0.10097867801338750343e1) * t46390 + F::new(0.18583473745796456084e3) * t12741 * t45812 * t1442 * t1507 - F::new(0.23967961564076583027e5) * t27552 * t54959 * t17921;
    t60235
}
