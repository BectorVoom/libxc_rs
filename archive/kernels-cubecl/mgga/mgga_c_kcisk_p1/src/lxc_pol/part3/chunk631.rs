//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 631/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk631<F: Float>(t2042: F, t2049: F, t240: F, t5212: F, t5215: F, t5221: F, t5340: F, t5525: F, t5527: F, t5532: F, t5533: F, t5552: F, t802: F) -> F {
    let t5556 = t5212 - t5215 + t5221 - t5340 + t240 * (-t2042 * t5552 - F::cast_from(2.0_f64) * t2049 * t5527 + t5525 * t802 + F::cast_from(2.0_f64) * t5532 * t5533 - t5212 + t5215 - t5221 + t5340);
    t5556
}
