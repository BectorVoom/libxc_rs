//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1054/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1054(t11535: f64, t11541: f64, t11550: f64, t11555: f64, t11559: f64, t11562: f64, t11592: f64, t11595: f64, t12129: f64, t12131: f64, t12135: f64, t12136: f64, t12137: f64, t12138: f64, t12139: f64, t12140: f64, t12141: f64, t12144: f64, t12145: f64, t12146: f64) -> f64 {
    let t12147 = 0.49166375783284505217e-7_f64 * t11535 - 0.22099262292595577329e-7_f64 * t11541 + t12129 - 0.252977417353824213e-7_f64 * t11550 - t12131 - 0.49166375783284505217e-8_f64 * t11555 + 0.32777583855523003478e-8_f64 * t11559 - 0.57970906942607043474e-5_f64 * t11562 + t12135 + t12136 - t12137 + t12138 - t12139 + t12140 - t12141 + 0.96684272530105650816e-8_f64 * t11592 + 0.90579542097823505425e-7_f64 * t11595 + t12144 + t12145 - t12146;
    t12147
}
