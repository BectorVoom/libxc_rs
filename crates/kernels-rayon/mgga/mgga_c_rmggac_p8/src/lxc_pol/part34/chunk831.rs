//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 831/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk831(t15252: f64, t495: f64, t515: f64, t7230: f64, t7231: f64, t3154: f64, t9090: f64, t13832: f64, t61965: f64, t13836: f64, t38495: f64, t321: f64, t3351: f64, t7248: f64) -> (f64, f64, f64, f64, f64) {
    let t74927 = 0.1064114997332445985e-4_f64 * t7230 * t7231 * t515 * t15252 * t495;
    let t74928 = t9090 * t3154;
    let t74929 = 0.19863479950205658386e-4_f64 * t74928;
    let t74930 = t61965 * t13832;
    let t74932 = t38495 * t13836;
    let t74943 = t3351 * t7248 * t515 * t15252 * t321;
    (t74927, t74929, t74930, t74932, t74943)
}
