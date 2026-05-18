//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 871/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk871<F: Float>(t13233: F, t13235: F, t13237: F, t13238: F, t13240: F, t13245: F, t13247: F, t13284: F, t13295: F, t13302: F, t13306: F, t13308: F, t13313: F) -> F {
    let t13671 = -t13233 - t13235 - t13237 + t13238 - t13240 + t13245 + t13247 + t13284 + t13295 + t13302 - t13306 - t13308 - t13313;
    t13671
}
