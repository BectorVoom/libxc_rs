//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1435/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1435<F: Float>(t10289: F, t10299: F, t10293: F, t10302: F, t10625: F, t10292: F, t11143: F, t10626: F, t31447: F, t31448: F, t31449: F, t31450: F, t31451: F, t31452: F, t31453: F, t31454: F, t31455: F, t32097: F, t35249: F, t7: F) -> F {
    let t35252 = F::cast_from(2.0_f64) * t10289;
    let t35253 = F::cast_from(4.0_f64) * t10299;
    let t35254 = F::cast_from(4.0_f64) * t10293;
    let t35255 = F::cast_from(4.0_f64) * t10302;
    let t35256 = F::cast_from(2.0_f64) * t10625;
    let t35257 = F::cast_from(2.0_f64) * t10292;
    let t35259 = F::cast_from(2.0_f64) * t11143;
    let tv4rho2sigma21 = -t31447 - t31448 + t31449 - t31450 + t31451 + t31452 - t31453 + t31454 - t31455 + t7 * (t32097 + t35249) - t35252 + t35253 + t35254 + t35255 - t35256 - t35257 + F::cast_from(2.0_f64) * t10626 + t35259;
    tv4rho2sigma21
}
