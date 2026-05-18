//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1187/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1187<F: Float>(t3717: F, t4066: F, t1105: F, t4227: F, t2376: F, t2409: F, t1205: F, t3721: F, t9296: F, t14605: F, t14867: F, t14888: F, t14914: F, t15036: F, t15135: F, t15147: F, t15152: F, t15162: F, t15165: F, t15170: F, t15178: F, t15183: F, t15187: F, t2408: F, t3066: F, t3917: F, t4083: F, t8793: F) -> (F, F, F, F, F, F) {
    let t15406 = t4066 * t3717;
    let t15423 = t4227 * t1105;
    let t15425 = t2409 * t2376 * t15423;
    let t15429 = t1205 * t3721;
    let t15431 = t2409 * t9296 * t15429;
    let t15437 = -t15135 / F::new(384.0) - t15147 / F::new(384.0) - t15152 / F::new(768.0) + t8793 * t14888 / F::new(24.0) + t8793 * t15036 / F::new(24.0) - F::new(7.0) / F::new(72.0) * t14867 + t15162 / F::new(48.0) + t15165 / F::new(24.0) + t15170 / F::new(768.0) - t15178 / F::new(1536.0) + t15183 / F::new(192.0) + F::new(7.0) / F::new(1152.0) * t14605 + t2408 * t15425 / F::new(24.0) + t15187 / F::new(768.0) - t3066 * t15431 / F::new(16.0) - t3917 * t4083 / F::new(96.0) + F::new(7.0) / F::new(144.0) * t14914;
    (t15406, t15423, t15425, t15429, t15431, t15437)
}
