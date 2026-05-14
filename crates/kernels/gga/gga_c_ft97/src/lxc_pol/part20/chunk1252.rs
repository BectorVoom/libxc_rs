//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1252/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1252<F: Float>(t113075: F, t113116: F, t113166: F, t113219: F, t113245: F, t113284: F, t113327: F, t113358: F, t113399: F, t113425: F, t113455: F, t113505: F, t113539: F, t113573: F, t113615: F, t113635: F, t871: F) -> (F,) {
    let t113640 = t871 * (t113075 + t113116 + t113166 + t113219 + t113245 + t113284 + t113327 + t113358 + t113399 + t113425 + t113455 + t113505 + t113539 + t113573 + t113615 + t113635);
    (t113640,)
}
