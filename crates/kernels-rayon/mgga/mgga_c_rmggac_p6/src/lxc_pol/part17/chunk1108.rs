//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1108/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1108(t10198: f64, t42335: f64, t42336: f64, t42337: f64, t42338: f64, t42339: f64, t42340: f64, t42341: f64, t42345: f64, t7938: f64, t7941: f64, t10201: f64, t10204: f64, t10206: f64, t42355: f64, t42356: f64, t42357: f64, t42358: f64, t42359: f64, t9231: f64, t9671: f64, t9672: f64) -> (f64, f64) {
    let t48086 = -t42335 + t42336 + t42337 + t42338 + t10198 - t42339 - t42340 + t42341 + t7938 - t7941 - t42345;
    let t48091 = t9671 + t10201 - t42355 - t9672 + 4.0_f64 * t9231 + t10204 - t42356 - t42357 - t42358 + t42359 - t10206;
    (t48086, t48091)
}
