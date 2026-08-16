//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1116/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1116(t7754: f64, t8450: f64, t31047: f64, t7687: f64, t1983: f64, t191: f64, t192: f64, t7681: f64, t2020: f64, t3701: f64, t7752: f64, t2019: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t33127 = t8450 * t7754;
    let t33129 = t31047 * t7687;
    let t33131 = 3.0_f64 * t1983 * t33129;
    let t33133 = t7681 * t191 * t192;
    let t33134 = t33133 * t2020;
    let t33136 = t3701 * t7752;
    let t33137 = t2019 * t33136;
    (t33127, t33129, t33131, t33133, t33134, t33136, t33137)
}
