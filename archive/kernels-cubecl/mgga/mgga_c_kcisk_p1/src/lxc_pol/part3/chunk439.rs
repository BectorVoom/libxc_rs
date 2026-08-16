//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 439/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk439<F: Float>(t1140: F, t1147: F, t289: F, t3180: F, t3183: F, t3189: F, t3275: F, t3435: F, t3437: F, t3442: F, t3443: F, t3460: F) -> F {
    let t3462 = -t1140 * t3460 - F::cast_from(2.0_f64) * t1147 * t3437 + t289 * t3435 + F::cast_from(2.0_f64) * t3442 * t3443 - t3180 + t3183 - t3189 + t3275;
    t3462
}
