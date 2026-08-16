//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2117/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2117<F: Float>(t14046: F, t25986: F, t2661: F, t14050: F, t13850: F, t2482: F, t25981: F, t814: F, t13962: F, t26028: F, t14020: F, t7252: F) -> (F, F, F, F, F) {
    let t98235 = t2661 * t25986 * t14046;
    let t98236 = F::cast_from(0.11433071498151929859e-3_f64) * t98235;
    let t98238 = t2661 * t25986 * t14050;
    let t98239 = F::cast_from(0.28582678745379824648e-4_f64) * t98238;
    let t98243 = t2482 * t25981 * t814 * t13850;
    let t98244 = F::cast_from(0.10164000561857065645e-3_f64) * t98243;
    let t98245 = t26028 * t13962;
    let t98253 = t7252 * t14020;
    (t98236, t98239, t98244, t98245, t98253)
}
