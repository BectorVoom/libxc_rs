//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1278/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1278(t26264: f64, t373: f64, t2942: f64, t2950: f64, t2958: f64, t1897: f64, t1900: f64, t8428: f64, t11: f64, t8620: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t26265 = 0.13388493827160493828e1_f64 * t26264;
    let t26266 = f64::powf(t373, -0.25e1_f64);
    let t26267 = t2942 * t2942;
    let t26268 = t26266 * t26267;
    let t26270 = t2950 * t2950;
    let t26271 = t2958 * t26270;
    let t26276 = t8428 * t1897 * t1900;
    let t26278 = t11 * t8620 * t26276;
    (t26265, t26267, t26268, t26270, t26271, t26276, t26278)
}
