//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1159/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1159<F: Float>(t168: F, t18347: F, t22758: F, t22760: F, t22766: F, t245: F, t34334: F, t34336: F, t34340: F, t34360: F, t42923: F, t42928: F, t42935: F, t47308: F, t47340: F, t47370: F, t47524: F, t47543: F, t47563: F, t47585: F, t47633: F, t47672: F, t47692: F, t47726: F, t47754: F, t47781: F, t47807: F, t47829: F, t47856: F, t47873: F, t47900: F, t47927: F, t48063: F, t48091: F, t48110: F, t48129: F, t48143: F, t48166: F, t48208: F, t48235: F, t48286: F, t48312: F, t48370: F, t48395: F, t48423: F) -> F {
    let t48434 = -F::cast_from(0.31835665774679373269e0_f64) * t34334 - F::cast_from(0.50257692321302641126e0_f64) * t34336 + F::cast_from(0.10051538464260528225e1_f64) * t34340 - F::cast_from(0.33505128214201760751e0_f64) * t42923 - F::cast_from(0.33505128214201760751e0_f64) * t22758 + F::cast_from(0.10051538464260528225e1_f64) * t22760 - F::cast_from(0.22778074678193449956e1_f64) * t42928 - F::cast_from(0.17716280305261572188e2_f64) * t22766 - t18347 - F::cast_from(0.11938374665504764976e-1_f64) * t168 * t245 * (t48208 + t47856 + t47754 + t47672 + t48286 + t47873 + t47726 + t47692 + t47524 + t47900 + t47781 + t48423 + t47543 + t48143 + t48312 + t48091 + t47585 + t47308 + t47807 + t47370 + t48063 + t48166 + t48129 + t47340 + t47829 + t47633 + t48370 + t48110 + t47563 + t47927 + t48395 + t48235) - F::cast_from(0.10051538464260528225e1_f64) * t34360 + F::cast_from(0.79589164436698433172e-1_f64) * t42935;
    t48434
}
