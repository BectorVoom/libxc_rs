//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 1002/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk1002(t13333: f64, t13334: f64, t13336: f64, t13338: f64, t13340: f64, t13342: f64, t13345: f64, t13348: f64, t13349: f64, t13352: f64, t13484: f64, t13723: f64, t46840: f64, t51215: f64, t7: f64) -> f64 {
    let tv4rhosigma313 = t13333 - t13334 - t13336 + t13338 - t13340 + t13342 - t13345 + t13348 - t13349 + t13352 - t13484 + t13723 + t7 * (t46840 + t51215);
    tv4rhosigma313
}
