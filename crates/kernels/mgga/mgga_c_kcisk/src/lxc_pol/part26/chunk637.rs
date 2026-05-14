//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 637/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk637<F: Float>(t1483: F, t2267: F, t1413: F, t2257: F, t1489: F, t1517: F, t2271: F, t6355: F, t6358: F, t6361: F, t6364: F, t6366: F, t6371: F, t6374: F, t6378: F, t6380: F, t6383: F, sigma0: F) -> (F, F, F, F, F, F) {
    let t6385 = t1483 * t2267;
    let t6387 = t2257 * t1413;
    let t6388 = t6387 * sigma0;
    let t6389 = t6388 * t1489;
    let t6391 = t2271 * t1517;
    let t6393 = -t6355 / 192.0 - t6358 / 24.0 + t6361 / 6.0 + t6364 / 36.0 - t6366 / 16.0 - t6371 / 128.0 + t6374 / 192.0 - t6378 / 16.0 + t6380 / 256.0 + t6383 / 256.0 + t6385 / 24.0 - t6389 / 16.0 - t6391 / 192.0;
    (t6385, t6387, t6388, t6389, t6391, t6393)
}
