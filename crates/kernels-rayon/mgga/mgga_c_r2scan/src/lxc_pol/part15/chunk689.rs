//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 689/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk689(t5222: f64, t615: f64, t1757: f64, t1726: f64, t611: f64, t1727: f64, t616: f64, t1745: f64, t378: f64, t735: f64, t1376: f64, t5: f64) -> (f64, f64, f64, f64) {
    let t5223 = t615 * t5222;
    let t5225 = 0.50808839199999999999e-2_f64 * t1757 * t5223;
    let t5226 = t1726 * t611;
    let t5227 = t616 * t1727;
    let t5228 = t615 * t5227;
    let t5230 = 0.1524265176e-1_f64 * t5226 * t5228;
    let t5231 = t378 * t1745;
    let t5233 = 0.16265371950452609763e-1_f64 * t735 * t5231;
    let t5234 = t1376 * t5;
    (t5225, t5230, t5233, t5234)
}
