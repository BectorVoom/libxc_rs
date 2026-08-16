//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2352/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2352<F: Float>(t20896: F, t2697: F, t13360: F, t5624: F, t1516: F, t58844: F, t5628: F, t67441: F, t842: F, t59263: F, t59276: F, t59279: F, t59282: F, t59288: F, t59298: F, t59308: F, t59310: F, t59322: F, t849: F) -> F {
    let t68195 = t2697 * t20896;
    let t68197 = t13360 * t5624;
    let t68199 = t58844 * t1516;
    let t68201 = t13360 * t5628;
    let t68203 = t67441 * t842;
    let t68207 = -F::cast_from(119.0_f64) / F::cast_from(1152.0_f64) * t59263 - F::cast_from(119.0_f64) / F::cast_from(4608.0_f64) * t59276 + F::cast_from(35.0_f64) / F::cast_from(64.0_f64) * t59279 + F::cast_from(7.0_f64) / F::cast_from(1536.0_f64) * t59282 + F::cast_from(119.0_f64) / F::cast_from(4608.0_f64) * t59288 - F::cast_from(7.0_f64) / F::cast_from(16.0_f64) * t59298 - F::cast_from(7.0_f64) / F::cast_from(1536.0_f64) * t59308 - F::cast_from(7.0_f64) / F::cast_from(8.0_f64) * t59310 + F::cast_from(35.0_f64) / F::cast_from(192.0_f64) * t68195 - F::cast_from(35.0_f64) / F::cast_from(384.0_f64) * t68197 + F::cast_from(7.0_f64) / F::cast_from(384.0_f64) * t68199 + F::cast_from(7.0_f64) / F::cast_from(384.0_f64) * t68201 - t68203 * t849 / F::cast_from(768.0_f64) + F::cast_from(35.0_f64) / F::cast_from(192.0_f64) * t59322;
    t68207
}
