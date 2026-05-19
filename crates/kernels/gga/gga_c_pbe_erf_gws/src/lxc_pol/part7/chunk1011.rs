//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1011/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1011<F: Float>(t18092: F, t242: F, t168: F, t18009: F, t18156: F, t18157: F, t18159: F, t18188: F, t18191: F, t18197: F, t18203: F, t18204: F, t18208: F, t18211: F, t18213: F, t18216: F, t18225: F, t18226: F, t18228: F, t18229: F, t18238: F, t18254: F, t18270: F, t18271: F, t18281: F, t18282: F, t18295: F, t18300: F, t18304: F, t18319: F, t18321: F, t18322: F, t18325: F, t18326: F, t18336: F, t18339: F, t18342: F, t18347: F, t18349: F, t18352: F, t18355: F, t18359: F, t18360: F, t18363: F, t245: F) -> F {
    let t18364 = t18092 * t242;
    let t18366 = -F::cast_from(0.11938374665504764976e-1_f64) * t168 * t245 * (t18300 + t18156 + t18157 + t18159 + t18188 + t18191 + t18270 + t18271 + t18295 + t18197 + t18211 + t18009 + t18208 + t18281 + t18282 + t18254 + t18304 + t18213 + t18216 + t18203 + t18204 + t18229 + t18238 + t18225 + t18226 + t18228 + t18319 + t18321 + t18322 + t18325 + t18326) - F::cast_from(0.31835665774679373269e0_f64) * t18336 - F::cast_from(0.22778074678193449956e1_f64) * t18339 + F::cast_from(0.79589164436698433172e-1_f64) * t18342 - t18347 - F::cast_from(0.17716280305261572188e2_f64) * t18349 + F::cast_from(0.77820516338105134659e0_f64) * t18352 + F::cast_from(0.79723261373677074846e1_f64) * t18355 + t18359 - F::cast_from(0.2010307692852105645e1_f64) * t18360 - t18363 + F::cast_from(0.33505128214201760751e0_f64) * t18364;
    t18366
}
