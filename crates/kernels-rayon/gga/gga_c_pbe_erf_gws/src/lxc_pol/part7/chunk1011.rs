//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1011/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1011(t18092: f64, t242: f64, t168: f64, t18009: f64, t18156: f64, t18157: f64, t18159: f64, t18188: f64, t18191: f64, t18197: f64, t18203: f64, t18204: f64, t18208: f64, t18211: f64, t18213: f64, t18216: f64, t18225: f64, t18226: f64, t18228: f64, t18229: f64, t18238: f64, t18254: f64, t18270: f64, t18271: f64, t18281: f64, t18282: f64, t18295: f64, t18300: f64, t18304: f64, t18319: f64, t18321: f64, t18322: f64, t18325: f64, t18326: f64, t18336: f64, t18339: f64, t18342: f64, t18347: f64, t18349: f64, t18352: f64, t18355: f64, t18359: f64, t18360: f64, t18363: f64, t245: f64) -> f64 {
    let t18364 = t18092 * t242;
    let t18366 = -0.11938374665504764976e-1_f64 * t168 * t245 * (t18300 + t18156 + t18157 + t18159 + t18188 + t18191 + t18270 + t18271 + t18295 + t18197 + t18211 + t18009 + t18208 + t18281 + t18282 + t18254 + t18304 + t18213 + t18216 + t18203 + t18204 + t18229 + t18238 + t18225 + t18226 + t18228 + t18319 + t18321 + t18322 + t18325 + t18326) - 0.31835665774679373269e0_f64 * t18336 - 0.22778074678193449956e1_f64 * t18339 + 0.79589164436698433172e-1_f64 * t18342 - t18347 - 0.17716280305261572188e2_f64 * t18349 + 0.77820516338105134659e0_f64 * t18352 + 0.79723261373677074846e1_f64 * t18355 + t18359 - 0.2010307692852105645e1_f64 * t18360 - t18363 + 0.33505128214201760751e0_f64 * t18364;
    t18366
}
