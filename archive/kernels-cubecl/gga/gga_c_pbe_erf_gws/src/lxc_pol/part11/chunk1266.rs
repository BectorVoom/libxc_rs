//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1266/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1266<F: Float>(t13293: F, t39095: F, t11808: F, t12054: F, t11984: F, t13491: F, t3180: F, t45248: F, t11464: F, t11514: F, t11994: F, t13335: F, t13340: F, t13347: F, t21361: F, t2255: F, t2277: F, t2343: F, t2345: F, t3219: F, t3235: F, t3247: F, t46151: F, t49374: F, t49853: F, t50002: F, t6555: F, t904: F, t916: F, t929: F) -> (F, F, F, F, F) {
    let t50158 = t39095 * t13293 / F::cast_from(16.0_f64);
    let t50160 = t12054 * t11808 / F::cast_from(8.0_f64);
    let t50162 = t11984 * t13491 / F::cast_from(24.0_f64);
    let t50168 = t45248 * t3180 / F::cast_from(12.0_f64);
    let t50181 = F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t46151 - F::cast_from(3.0_f64) / F::cast_from(64.0_f64) * t6555 * t916 * t904 * t49853 + F::cast_from(35.0_f64) / F::cast_from(128.0_f64) * t929 * t21361 * t904 * t50002 + t50158 - t50160 - t50162 - t2277 * t2255 * t11994 * t13340 / F::cast_from(512.0_f64) - t50168 - t2343 * t3235 * t3219 * t13335 / F::cast_from(384.0_f64) + t2343 * t2345 * t11464 * t13347 / F::cast_from(64.0_f64) - F::cast_from(3.0_f64) / F::cast_from(64.0_f64) * t3247 * t2345 * t11514 * t49374;
    (t50158, t50160, t50162, t50168, t50181)
}
