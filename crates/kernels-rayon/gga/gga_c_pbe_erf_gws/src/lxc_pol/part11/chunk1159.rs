//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1159/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1159(t168: f64, t18347: f64, t22758: f64, t22760: f64, t22766: f64, t245: f64, t34334: f64, t34336: f64, t34340: f64, t34360: f64, t42923: f64, t42928: f64, t42935: f64, t47308: f64, t47340: f64, t47370: f64, t47524: f64, t47543: f64, t47563: f64, t47585: f64, t47633: f64, t47672: f64, t47692: f64, t47726: f64, t47754: f64, t47781: f64, t47807: f64, t47829: f64, t47856: f64, t47873: f64, t47900: f64, t47927: f64, t48063: f64, t48091: f64, t48110: f64, t48129: f64, t48143: f64, t48166: f64, t48208: f64, t48235: f64, t48286: f64, t48312: f64, t48370: f64, t48395: f64, t48423: f64) -> f64 {
    let t48434 = -0.31835665774679373269e0_f64 * t34334 - 0.50257692321302641126e0_f64 * t34336 + 0.10051538464260528225e1_f64 * t34340 - 0.33505128214201760751e0_f64 * t42923 - 0.33505128214201760751e0_f64 * t22758 + 0.10051538464260528225e1_f64 * t22760 - 0.22778074678193449956e1_f64 * t42928 - 0.17716280305261572188e2_f64 * t22766 - t18347 - 0.11938374665504764976e-1_f64 * t168 * t245 * (t48208 + t47856 + t47754 + t47672 + t48286 + t47873 + t47726 + t47692 + t47524 + t47900 + t47781 + t48423 + t47543 + t48143 + t48312 + t48091 + t47585 + t47308 + t47807 + t47370 + t48063 + t48166 + t48129 + t47340 + t47829 + t47633 + t48370 + t48110 + t47563 + t47927 + t48395 + t48235) - 0.10051538464260528225e1_f64 * t34360 + 0.79589164436698433172e-1_f64 * t42935;
    t48434
}
