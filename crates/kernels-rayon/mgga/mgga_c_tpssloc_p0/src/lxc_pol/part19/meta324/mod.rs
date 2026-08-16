//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta324 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1152;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1153;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta324(t32253: f64, t59: f64, t154: f64, t541: f64, t3850: f64, t550: f64, t12289: f64, t1336: f64, t835: f64, t12293: f64, t12364: f64, t3777: f64, t1354: f64, t12365: f64, t3853: f64, t12267: f64, t3789: f64, t3798: f64, t12297: f64, t12385: f64, t12300: f64, t3858: f64, t12402: f64, t12407: f64, t12409: f64, t12413: f64, t12429: f64, t1341: f64, t1343: f64, t3795: f64, t3803: f64, t3805: f64, t820: f64) -> (f64, f64, f64, f64, f64) {
        let (t39933, t39934, t39936, t39937, t39938, t39945, t39947) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1152(t32253, t59, t154, t541, t3850, t550, t12289, t1336, t835, t12293, t12364, t3777);
        let t39970 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1153(t1354, t39947, t12365, t3853, t12267, t3789, t3798, t12297, t12385, t12300, t3858, t12402, t12407, t12409, t12413, t12429, t1341, t1343, t3795, t3803, t3805, t39936, t39938, t39945, t820);
    (t39933, t39934, t39937, t39938, t39970)
}
