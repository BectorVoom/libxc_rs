//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1372/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1372(t15139: f64, t22493: f64, t11624: f64, t13917: f64, t51066: f64, t13888: f64, t353: f64, t3886: f64, t859: f64, t2249: f64, t56296: f64, t11541: f64) -> (f64, f64, f64, f64) {
    let t57581 = t22493 * t15139;
    let t57584 = t13917 * t51066 * t11624;
    let t57588 = t859 * t353 * t13888 * t3886;
    let t57591 = t2249 * t56296;
    let t57593 = t13917 * t57591 * t11541;
    (t57581, t57584, t57588, t57593)
}
