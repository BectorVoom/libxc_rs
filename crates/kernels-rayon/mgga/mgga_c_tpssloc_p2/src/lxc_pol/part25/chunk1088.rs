//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1088/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1088(t22715: f64, t547: f64, t1329: f64, t22822: f64, t281: f64, t6924: f64, t22794: f64, t120: f64, t22816: f64, t22814: f64, t22855: f64, t236: f64, t3791: f64) -> (f64, f64, f64, f64, f64) {
    let t80775 = t22715 * t547;
    let t80776 = t80775 * t1329;
    let t80779 = t22822 * t6924 * t281;
    let t80780 = t80779 * t22794;
    let t80782 = t22816 * t120;
    let t80783 = t22814 * t80782;
    let t80784 = t80783 * t22855;
    let t80786 = t236 * t3791;
    (t80776, t80780, t80782, t80784, t80786)
}
