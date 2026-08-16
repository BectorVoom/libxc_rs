//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 949/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk949(t1873: f64, t2759: f64, t667: f64, t1867: f64, t2765: f64, t1073: f64, t5511: f64, t1862: f64, t5547: f64, t5522: f64, t7332: f64, t7336: f64, t7352: f64, t7361: f64, t7363: f64, t7366: f64, t7368: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7370 = t1873 * t2759;
    let t7371 = t7370 * t667;
    let t7373 = t2765 * t1867;
    let t7375 = t5511 * t1073;
    let t7376 = t7375 * t1862;
    let t7378 = t5547 * t1073;
    let t7379 = t7378 * t1862;
    let t7382 = 0.34731666666666666667e0_f64 * t7332 - t7336 + 0.1549425e1_f64 * t7352 + 0.6311625e0_f64 * t7361 + 0.3529725e1_f64 * t7363 - 0.3529725e1_f64 * t7366 - 0.17648625e1_f64 * t7368 + 0.6311625e0_f64 * t7371 + 0.31558125e0_f64 * t7373 + 0.264729375e1_f64 * t7376 - 0.157790625e0_f64 * t7379 + 0.13772666666666666667e1_f64 * t5522;
    (t7370, t7371, t7373, t7375, t7376, t7378, t7379, t7382)
}
