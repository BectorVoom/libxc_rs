//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1070/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1070(t46849: f64, t6508: f64, t1358: f64, t6507: f64, t42529: f64, t42533: f64, t42537: f64, t42540: f64, t42544: f64, t42547: f64, t42551: f64, t42570: f64, t42573: f64, t42575: f64) -> (f64, f64) {
    let t46850 = t6508 * t46849;
    let t46852 = t1358 * t6507 * t46850;
    let t46857 = -0.63233348079280332442e-2_f64 * t46852 + 0.11856252764865062333e-2_f64 * t42529 - 0.31616674039640166221e-2_f64 * t42533 + t42537 + t42540 + t42544 - t42547 - t42551 - t42570 - t42573 + 0.94850022118920498663e-2_f64 * t42575;
    (t46850, t46857)
}
