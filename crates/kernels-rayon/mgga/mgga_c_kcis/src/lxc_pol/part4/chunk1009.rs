//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1009/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1009(t4277: f64, t3728: f64, t3739: f64, t1466: f64, t4108: f64, t3735: f64, t4142: f64, t4149: f64, t2820: f64, t3751: f64, t86: f64, t4155: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11776 = t4277 * sigma2;
    let t11780 = t3728 * t3739;
    let t11782 = t4108 * t1466;
    let t11783 = t11782 * sigma2;
    let t11799 = t3728 * t3735;
    let t11811 = t4142 * t4149;
    let t11814 = t86 * t2820 * t3751;
    let t11815 = t11814 * t4155;
    (t11776, t11780, t11782, t11783, t11799, t11811, t11815)
}
