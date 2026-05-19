//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1256/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1256<F: Float>(t40312: F, t37431: F, t37438: F, t40294: F, t40303: F, t40305: F, t40308: F, t40315: F, t40319: F, t41280: F, t41283: F, t41285: F, t41286: F, t41289: F, t41291: F) -> F {
    let t42187 = F::cast_from(0.1440846329149835838e-2_f64) * t40312;
    let t42192 = -t41280 + t41283 - t41285 - F::cast_from(0.72042316457491791901e-3_f64) * t40294 - F::cast_from(0.3842256877732895568e-2_f64) * t40303 + F::cast_from(0.92232789896410962669e-3_f64) * t40305 + F::cast_from(0.72042316457491791901e-3_f64) * t40308 + t42187 - F::cast_from(0.86737941314158990616e-4_f64) * t40315 - F::cast_from(0.20496175532535769482e-3_f64) * t40319 - t41286 + t41289 + t41291 - F::cast_from(0.2881692658299671676e-2_f64) * t37431 + F::cast_from(0.40992351065071538965e-3_f64) * t37438;
    t42192
}
