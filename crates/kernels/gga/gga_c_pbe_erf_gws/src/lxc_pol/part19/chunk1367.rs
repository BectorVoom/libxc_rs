//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1367/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1367<F: Float>(t1161: F, t353: F, t55151: F, t859: F, t11363: F, t1206: F, t14911: F, t2498: F, t52353: F, t55279: F, t55290: F, t55311: F, t56647: F, t56651: F, t56657: F, t56667: F, t56674: F, t56678: F, t56686: F, t56697: F, t56701: F, t6793: F, t9241: F, t9283: F) -> F {
    let t58292 = t859 * t353 * t55151 * t1161;
    let t58302 = -t2498 * t14911 / F::cast_from(48.0_f64) + t56647 / F::cast_from(192.0_f64) - t56651 / F::cast_from(384.0_f64) + t56657 / F::cast_from(384.0_f64) + t55279 + t56667 / F::cast_from(192.0_f64) - t56674 / F::cast_from(24.0_f64) - t56678 / F::cast_from(192.0_f64) - t56686 / F::cast_from(768.0_f64) + t6793 * t58292 / F::cast_from(24.0_f64) + t55290 - F::cast_from(35.0_f64) / F::cast_from(432.0_f64) * t52353 - t56697 / F::cast_from(768.0_f64) - t56701 / F::cast_from(1536.0_f64) + t9241 * t9283 * t1206 * t11363 / F::cast_from(4.0_f64) - t55311;
    t58302
}
