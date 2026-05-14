//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 441/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk441<F: Float>(t5393: F, t871: F, t296: F, t319: F, t5299: F, t840: F, t1212: F, t1255: F, t992: F) -> (F, F, F, F) {
    let t5394 = t871 * t5393;
    let t5395 = t296 * t5394;
    let t5399 = t840 * t319 * t5299;
    let t5403 = t840 * t1255 * t1212;
    let t5408 = t992 * t1212;
    (t5395, t5399, t5403, t5408)
}
