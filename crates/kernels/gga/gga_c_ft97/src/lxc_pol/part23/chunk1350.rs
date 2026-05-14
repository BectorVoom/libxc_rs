//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1350/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1350<F: Float>(t113190: F, t113191: F, t19006: F, t113373: F, t113379: F, t113386: F, t126914: F, t126916: F, t126919: F, t126923: F, t126927: F, t126930: F, t126932: F, t126935: F, t1234: F, t668: F) -> (F, F, F) {
    let t126938 = t113190 * t113191 * t19006;
    let t126942 = t113373 - t126914 - t126916 + t126919 / 6.0 + t126923 / 6.0 + t126927 / 9.0 - t126930 + 2.0 * t126932 + 4.0 / 3.0 * t126935 - 4.0 / 9.0 * t126938 + 8.0 / 9.0 * t113379 + 2.0 / 9.0 * t113386;
    let t126946 = t1234 * t668;
    (t126938, t126942, t126946)
}
