//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 690/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk690(t3352: f64, t9117: f64, t7230: f64, t236: f64, t495: f64, t618: f64, t7231: f64, t2061: f64, t2868: f64, t117: f64, t6477: f64, t2295: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9118 = t3352 * t9117;
    let t9119 = t7230 * t9118;
    let t9122 = t236 * t618 * t495;
    let t9123 = t7231 * t9122;
    let t9124 = t7230 * t9123;
    let t9126 = t2868 * t2061;
    let t9128 = t6477 * t117;
    let t9129 = t9128 * t2295;
    (t9118, t9119, t9123, t9124, t9126, t9128, t9129)
}
