//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta395 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1539;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1540;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta395(t16225: f64, t3807: f64, t16224: f64, t12289: f64, t242: f64, t1336: f64, t16048: f64, t5248: f64, t5249: f64, t12283: f64, t5259: f64, t5293: f64, t120: f64, t5286: f64, t3805: f64, t12407: f64, t12284: f64, t12301: f64, t12397: f64, t12429: f64, t1341: f64, t1363: f64, t16147: f64, t16150: f64, t16155: f64, t16159: f64, t16208: f64, t16211: f64, t16214: f64, t16217: f64, t1827: f64, t3778: f64, t3803: f64, t5289: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t16226, t16227, t16233, t16235, t16239, t16241) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1539(t16225, t3807, t16224, t12289, t242, t1336, t16048, t5248, t5249, t12283, t5259, t5293);
        let (t16242, t16244, t16248, t16253) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1540(t120, t5286, t3805, t3807, t12407, t5249, t12284, t12301, t12397, t12429, t1341, t1363, t16147, t16150, t16155, t16159, t16208, t16211, t16214, t16217, t16227, t16233, t16235, t16239, t16241, t1827, t3778, t3803, t5259, t5289);
    (t16226, t16227, t16235, t16242, t16244, t16248, t16253)
}
