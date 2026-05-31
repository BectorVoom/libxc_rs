//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 858/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk858<F: Float>(t7268: F, t7312: F, t650: F, t186: F, t211: F, t2730: F, t2737: F, t1024: F, t5343: F, t5205: F, t7184: F, t7185: F, t7187: F, t7190: F, t7193: F, t7198: F, t7203: F, t7208: F, t7215: F, t7221: F, t7223: F, t7228: F, t7230: F) -> (F, F, F, F) {
    let t7313 = t7268 + t7312;
    let t7314 = t650 * t7313;
    let t7315 = t186 * t7314;
    let t7317 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t211 * t7315;
    let t7319 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t2730 * t2737;
    let t7321 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t5343 * t1024;
    let t7322 = t7184 + t7185 + F::cast_from(4.0_f64) / F::cast_from(135.0_f64) * t5205 + t7187 - t7190 + t7193 - t7198 + t7203 + t7208 - t7215 + t7221 + t7223 - t7228 + t7230 - t7317 - t7319 + t7321;
    (t7317, t7319, t7321, t7322)
}
