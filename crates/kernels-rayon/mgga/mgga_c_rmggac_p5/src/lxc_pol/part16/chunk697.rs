//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 697/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk697(t10066: f64, t7453: f64, t209: f64, t605: f64, t618: f64, t236: f64, t7231: f64, t1970: f64, t551: f64, t3352: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10067 = t7453 * t10066;
    let t10070 = t618 * t605 * t209;
    let t10071 = t236 * t10070;
    let t10072 = t7231 * t10071;
    let t10073 = t1970 * t10072;
    let t10076 = t551 * t605 * t209;
    let t10077 = t236 * t10076;
    let t10078 = t3352 * t10077;
    let t10079 = t1970 * t10078;
    let t10082 = t618 * t551;
    (t10067, t10072, t10073, t10078, t10079, t10082)
}
