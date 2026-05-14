//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1095/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1095<F: Float>(t108624: F, t108784: F, t108794: F, t108797: F, t108806: F, t108810: F, t108819: F, t108823: F, t108826: F, t108830: F, t108834: F, t108838: F, t1096: F, t13522: F, t13523: F, t18: F, t2248: F, t2405: F, t24276: F, t24280: F, t27500: F, t35415: F, t35456: F, t3759: F, t6014: F, t65694: F, t679: F, t689: F, t704: F, t96537: F, t96593: F) -> (F,) {
    let t108841 = -0.12020514968855939808e-5 * t65694 * t108784 + 0.98978452595430188146e-4 * t24276 * t96593 * t1096 * t2405 + t108794 - 0.39591381038172075259e-3 * t108797 * t24280 - 0.29693535778629056444e-3 * t24276 * t2248 * t704 * t18 * t679 * t689 + 0.26724182200766150799e-2 * t108806 * t35415 * t13522 - 0.21120586720831816188e-4 * t108810 * t35456 * t13522 - 0.23254900946437792e-1 * t3759 * t6014 * t108624 + 0.3404992446913580247e-1 * t27500 * t108819 - 0.49489226297715094073e-4 * t108823 + 0.27568129967481981592e-3 * t108826 * t13523 + 0.12768721675925925926e-1 * t27500 * t108830 + 0.51074886703703703704e-1 * t27500 * t108834 - 0.85124811172839506174e-2 * t108838 + 0.28374937057613168724e-2 * t96537;
    (t108841,)
}
