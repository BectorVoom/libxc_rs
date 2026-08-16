//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1044/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1044<F: Float>(t12393: F, t422: F, t12295: F, t12292: F, t12297: F, t12299: F, t12301: F, t12303: F, t12307: F, t12310: F, t12314: F, t12317: F, t12320: F) -> (F, F) {
    let t12395 = F::cast_from(0.621814e-1_f64) * t12393 * t422;
    let t12397 = F::cast_from(0.53272592592592592592e-1_f64) * t12295;
    let t12408 = -t12397 + F::cast_from(0.2283111111111111111e-1_f64) * t12297 + F::cast_from(0.11415555555555555555e-1_f64) * t12299 - F::cast_from(0.34246666666666666665e-1_f64) * t12301 - F::cast_from(0.17123333333333333333e-1_f64) * t12303 + F::cast_from(0.19025925925925925925e-1_f64) * t12307 - F::cast_from(0.68493333333333333331e-1_f64) * t12310 - F::cast_from(0.34246666666666666665e-1_f64) * t12292 + F::cast_from(0.10274e0_f64) * t12314 + F::cast_from(0.10274e0_f64) * t12317 + F::cast_from(0.17123333333333333333e-1_f64) * t12320;
    (t12395, t12408)
}
