//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1050/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1050(t1140: f64, t4645: f64, t1137: f64, t4632: f64, t13273: f64, t515: f64, t1456: f64, t3237: f64, t4759: f64, t997: f64, t4518: f64, t4574: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t18336 = t1140 * t4645;
    let t18338 = t1137 * t4632;
    let t18340 = t13273 * t515;
    let t18347 = t3237 * t1456;
    let t18349 = t997 * t4759;
    let t18351 = t997 * t4518;
    let t18364 = t1137 * t4574;
    (t18336, t18338, t18340, t18347, t18349, t18351, t18364)
}
