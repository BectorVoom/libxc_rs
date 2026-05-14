//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1049/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1049<F: Float>(t2371: F, t93: F, t1514: F, t2289: F, t4264: F, t625: F, t4288: F, t10208: F, t1513: F, t2340: F, t2339: F, t4287: F, t665: F, t2366: F, t4263: F, t10227: F, t1504: F, t2350: F) -> (F, F, F, F, F, F, F, F) {
    let t13440 = t93 * t2371;
    let t13448 = t2289 * t1514;
    let t13451 = 4.0 / 3.0 * t625 * t4264;
    let t13453 = 2.0 / 3.0 * t625 * t4288;
    let t13455 = t10208 * t1513 * t2340;
    let t13458 = t2339 * t4287;
    let t13459 = t13458 * t665;
    let t13462 = t4263 * t2366;
    let t13472 = t10227 * t1504 * t2350;
    (t13440, t13448, t13451, t13453, t13455, t13459, t13462, t13472)
}
