//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 706/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk706(t10070: f64, t236: f64, t7231: f64, t1970: f64, t209: f64, t551: f64, t605: f64, t3352: f64, t618: f64) -> (f64, f64, f64, f64, f64) {
    let t10071 = t236 * t10070;
    let t10072 = t7231 * t10071;
    let t10073 = t1970 * t10072;
    let t10074 = 0.85129199786595678796e-5_f64 * t10073;
    let t10076 = t551 * t605 * t209;
    let t10077 = t236 * t10076;
    let t10078 = t3352 * t10077;
    let t10079 = t1970 * t10078;
    let t10080 = 0.25538759935978703638e-4_f64 * t10079;
    let t10082 = t618 * t551;
    (t10072, t10074, t10078, t10080, t10082)
}
