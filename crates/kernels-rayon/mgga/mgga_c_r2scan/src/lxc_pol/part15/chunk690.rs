//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 690/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk690(t5234: f64, t736: f64, t1751: f64, t1754: f64, t159: f64, t166: f64, t15: f64, t3: f64, t42: f64, t148: f64, t40: f64, t1725: f64, t58: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5235 = t5234 * t736;
    let t5237 = t1751 * t1754;
    let t5239 = t159 * t166;
    let t5243 = 1.0_f64 / t15 / t3 / t42 / 48.0_f64;
    let t5244 = t148 * t5243;
    let t5245 = t3 * t40;
    let t5246 = t5244 * t5245;
    let t5248 = 0.42340699333333333333e-2_f64 * t5239 * t5246;
    let t5249 = t1725 * t58;
    (t5235, t5237, t5245, t5246, t5248, t5249)
}
