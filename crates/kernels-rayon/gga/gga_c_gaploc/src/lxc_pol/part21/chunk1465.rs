//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1465/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1465(t12033: f64, t12036: f64, t12150: f64, t31447: f64, t31448: f64, t31449: f64, t31453: f64, t31454: f64, t35252: f64, t35253: f64, t35254: f64, t35255: f64, t35257: f64, t38262: f64, t38263: f64, t38264: f64, t38266: f64, t38875: f64, t39526: f64, t7: f64) -> f64 {
    let t39529 = 2.0_f64 * t12033;
    let t39530 = 4.0_f64 * t12036;
    let tv4rho2sigma24 = -t31447 - t31448 + t31449 - t31453 + t31454 - t38262 + t38263 - t35252 - t38264 + t35253 + t35254 + t35255 + 2.0_f64 * t12150 + t38266 + t7 * (t38875 + t39526) - t39529 - t35257 + t39530;
    tv4rho2sigma24
}
