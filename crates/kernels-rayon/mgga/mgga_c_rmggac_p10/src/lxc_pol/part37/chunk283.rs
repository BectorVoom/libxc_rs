//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 283/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk283(t2190: f64, t674: f64, t2065: f64, t2086: f64, t321: f64, t699: f64, t305: f64, t333: f64, t326: f64, t698: f64, t874: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2191 = t2190 * t674;
    let t2200 = 0.79828278012425390427e-1_f64 * t2065;
    let t2204 = 0.18183107769496894487e-1_f64 * t2086;
    let t2205 = t699 * t321;
    let t2206 = t305 * t2205;
    let t2208 = t699 * t333;
    let t2209 = t326 * t2208;
    let t2211 = t874 * t698;
    (t2191, t2200, t2204, t2206, t2209, t2211)
}
