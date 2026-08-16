//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 529/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk529(t2067: f64, t2565: f64, t14249: f64, t14236: f64, t10897: f64, t2078: f64, t3369: f64, t3148: f64, t7472: f64) -> (f64, f64, f64, f64, f64) {
    let t14250 = t2067 * t2565;
    let t14251 = t14249 * t14250;
    let t14252 = t14236 * t14251;
    let t14254 = t2078 * t10897;
    let t14255 = t3369 * t14254;
    let t14256 = t14236 * t14255;
    let t14258 = t7472 * t3148;
    (t14251, t14252, t14255, t14256, t14258)
}
