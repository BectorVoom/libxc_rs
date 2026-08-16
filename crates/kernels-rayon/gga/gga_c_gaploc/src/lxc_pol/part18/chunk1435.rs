//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1435/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1435(t10289: f64, t10299: f64, t10293: f64, t10302: f64, t10625: f64, t10292: f64, t11143: f64, t10626: f64, t31447: f64, t31448: f64, t31449: f64, t31450: f64, t31451: f64, t31452: f64, t31453: f64, t31454: f64, t31455: f64, t32097: f64, t35249: f64, t7: f64) -> f64 {
    let t35252 = 2.0_f64 * t10289;
    let t35253 = 4.0_f64 * t10299;
    let t35254 = 4.0_f64 * t10293;
    let t35255 = 4.0_f64 * t10302;
    let t35256 = 2.0_f64 * t10625;
    let t35257 = 2.0_f64 * t10292;
    let t35259 = 2.0_f64 * t11143;
    let tv4rho2sigma21 = -t31447 - t31448 + t31449 - t31450 + t31451 + t31452 - t31453 + t31454 - t31455 + t7 * (t32097 + t35249) - t35252 + t35253 + t35254 + t35255 - t35256 - t35257 + 2.0_f64 * t10626 + t35259;
    tv4rho2sigma21
}
