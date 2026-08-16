//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1438/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1438(t10626: f64, t12333: f64, t31447: f64, t31448: f64, t31449: f64, t31450: f64, t31451: f64, t31452: f64, t31453: f64, t31454: f64, t31455: f64, t35252: f64, t35253: f64, t35254: f64, t35255: f64, t35256: f64, t35257: f64, t35259: f64, t39539: f64, t39540: f64, t7: f64) -> f64 {
    let tv4rho2sigma27 = -t31447 - t31448 + t31449 - t31450 + t31451 + t31452 - t31453 + t7 * (t39539 + t39540) + t31454 - t31455 - t35252 + t35253 + t12333 + t35254 + t35255 - t35256 - t35257 + t10626 + t35259;
    tv4rho2sigma27
}
