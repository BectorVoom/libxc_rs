//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1182/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1182<F: Float>(t240: F, t32225: F, t32228: F, t32231: F, t32232: F, t32233: F, t32235: F, t32237: F, t32240: F, t32243: F, t32246: F, t32309: F, t32520: F, t32536: F, t297: F, t294: F) -> (F, F) {
    let t32539 = t32225 - t32228 + t32231 - t32232 - t32233 + t32235 - t32237 - t32240 + t32243 + t32246 - t32309 + t240 * (t32520 + t32536);
    let t32540 = t297 * t32539;
    let t32541 = t294 * t32540;
    (t32539, t32541)
}
