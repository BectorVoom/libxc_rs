//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1107/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1107<F: Float>(t11808: F, t12054: F, t11984: F, t13491: F, t3180: F, t45248: F, t11464: F, t11514: F, t11994: F, t13335: F, t13340: F, t13347: F, t21361: F, t2255: F, t2277: F, t2343: F, t2345: F, t3219: F, t3235: F, t3247: F, t46151: F, t49374: F, t49853: F, t50002: F, t50158: F, t6555: F, t904: F, t916: F, t929: F) -> (F, F, F, F) {
    let t50160 = t12054 * t11808 / 8.0;
    let t50162 = t11984 * t13491 / 24.0;
    let t50168 = t45248 * t3180 / 12.0;
    let t50181 = 7.0 / 48.0 * t46151 - 3.0 / 64.0 * t6555 * t916 * t904 * t49853 + 35.0 / 128.0 * t929 * t21361 * t904 * t50002 + t50158 - t50160 - t50162 - t2277 * t2255 * t11994 * t13340 / 512.0 - t50168 - t2343 * t3235 * t3219 * t13335 / 384.0 + t2343 * t2345 * t11464 * t13347 / 64.0 - 3.0 / 64.0 * t3247 * t2345 * t11514 * t49374;
    (t50160, t50162, t50168, t50181)
}
