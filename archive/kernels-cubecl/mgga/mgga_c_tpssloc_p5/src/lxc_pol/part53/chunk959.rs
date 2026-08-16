//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 959/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk959<F: Float>(t32244: F, t9231: F, t116904: F, t2240: F, t12461: F, t8807: F, t111: F, t32262: F, t3701: F, t8803: F, t115305: F, t115330: F) -> (F, F, F, F, F, F, F) {
    let t116947 = t9231 * t32244;
    let t116954 = t2240 * t116904;
    let t117006 = t8807 * t12461;
    let t117014 = t32262 * t111;
    let t117084 = t8803 * t3701;
    let t117128 = F::cast_from(0.25587863262083522346e0_f64) * t115305;
    let t117133 = F::cast_from(0.3289868133696452873e-1_f64) * t115330;
    (t116947, t116954, t117006, t117014, t117084, t117128, t117133)
}
