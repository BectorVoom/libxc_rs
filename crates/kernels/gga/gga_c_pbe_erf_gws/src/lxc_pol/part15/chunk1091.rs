//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1091/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1091<F: Float>(t14404: F, t19906: F, t13917: F, t3258: F, t51021: F, t51023: F, t1114: F, t50942: F, t13984: F, t1192: F, t13835: F, t14420: F, t14622: F, t19704: F, t20113: F, t2376: F, t2408: F, t2409: F, t3066: F, t51030: F, t53187: F, t53189: F, t53199: F, t53207: F, t53212: F, t53220: F, t6793: F, t8574: F, t8589: F, t8734: F) -> (F, F) {
    let t53224 = 7.0 / 72.0 * t19906 * t14404;
    let t53227 = t13917 * t51021 * t3258 * t51023;
    let t53229 = t1114 * t50942;
    let t53230 = t53229 * t13984;
    let t53231 = 7.0 / 144.0 * t53230;
    let t53232 = -t53187 - t53189 + t2408 * t2409 * t8589 * t13835 / 24.0 + t3066 * t2409 * t8734 * t14622 / 24.0 - t53199 + t2408 * t2409 * t2376 * t1192 * t8574 / 48.0 - 5.0 / 768.0 * t53207 + 7.0 / 48.0 * t51030 + t53212 / 384.0 + t19704 * t14420 / 48.0 + t20113 * t14420 / 48.0 + t6793 * t53220 / 24.0 - t53224 + t53227 / 768.0 + t53231;
    (t53229, t53232)
}
