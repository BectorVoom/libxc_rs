//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1094/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1094<F: Float>(t3886: F, t810: F, t2376: F, t2409: F, t3893: F, t840: F, t3721: F, t3067: F, t1115: F, t12199: F, t12201: F, t12206: F, t12210: F, t12215: F, t12220: F, t12223: F, t12229: F, t12234: F, t12239: F, t12243: F, t12246: F, t2408: F, t3066: F, t3079: F, t3207: F, t827: F, t833: F, t9695: F, t9701: F, t9709: F) -> (F, F, F, F, F) {
    let t12248 = t3886 * t810;
    let t12250 = t2409 * t2376 * t12248;
    let t12253 = t840 * t3893;
    let t12255 = t3721 * t810;
    let t12257 = t2409 * t3067 * t12255;
    let t12260 = -F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t12199 + t12201 * t833 / F::cast_from(96.0_f64) - t3066 * t12206 / F::cast_from(16.0_f64) + t3207 * t12210 / F::cast_from(8.0_f64) + t3066 * t12215 / F::cast_from(24.0_f64) + t12220 * t3079 / F::cast_from(96.0_f64) + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t12223 - t1115 * t9709 / F::cast_from(48.0_f64) + t12229 * t3079 / F::cast_from(96.0_f64) + t827 * t12234 / F::cast_from(96.0_f64) + t9695 - t9701 - t3207 * t12239 / F::cast_from(16.0_f64) + t3207 * t12243 / F::cast_from(16.0_f64) + F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t12246 + t2408 * t12250 / F::cast_from(48.0_f64) + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t12253 - t2408 * t12257 / F::cast_from(24.0_f64);
    (t12248, t12250, t12255, t12257, t12260)
}
