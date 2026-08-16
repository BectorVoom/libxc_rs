//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1152/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1152(t32253: f64, t59: f64, t154: f64, t541: f64, t3850: f64, t550: f64, t12289: f64, t1336: f64, t835: f64, t12293: f64, t12364: f64, t3777: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t39933 = t59 * t32253;
    let t39934 = t39933 * t154;
    let t39936 = 455.0_f64 / 243.0_f64 * t39934 * t541;
    let t39937 = t3850 * t3850;
    let t39938 = t39937 * t550;
    let t39944 = t1336 * t12289 * t835;
    let t39945 = t39944 * t12293;
    let t39947 = t3777 * t12364;
    (t39933, t39934, t39936, t39937, t39938, t39945, t39947)
}
