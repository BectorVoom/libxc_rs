//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 533/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk533(t495: f64, t498: f64, t236: f64, t7231: f64, t7230: f64, t321: f64, t3352: f64, t464: f64, t483: f64, t1968: f64, t1966: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7232 = t495 * t498;
    let t7233 = t236 * t7232;
    let t7234 = t7231 * t7233;
    let t7235 = t7230 * t7234;
    let t7236 = 0.1064114997332445985e-4_f64 * t7235;
    let t7237 = t495 * t321;
    let t7238 = t236 * t7237;
    let t7239 = t3352 * t7238;
    let t7240 = t7230 * t7239;
    let t7241 = 0.31923449919973379548e-4_f64 * t7240;
    let t7242 = t464 * t483;
    let t7243 = t7242 * t1968;
    let t7244 = t1966 * t7243;
    (t7234, t7236, t7239, t7241, t7243, t7244)
}
