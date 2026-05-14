//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1295/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1295<F: Float>(t2475: F, t31701: F, t7403: F, t11494: F, t7371: F, t27111: F, t27384: F, t956: F, t1003: F, t11563: F, t11567: F, t2575: F, t31682: F, t31685: F, t31688: F, t31691: F, t31693: F, t31697: F, t31700: F, t3608: F, t9305: F) -> (F, F, F, F) {
    let t31704 = 0.51726012919273400301e3 * t7403 * t31701 * t2475;
    let t31706 = 0.64327917994770140268e2 * t7371 * t11494;
    let t31709 = 0.2069040516770936012e4 * t27384 * t27111 * t956;
    let t31718 = -t31682 - t31685 - t31688 - t31691 + t31693 + t31697 + t31700 + t31704 + t31706 + t31709 + 0.14035736694323150897e2 * t1003 * t11563 * t2575 + 0.2077903092681775651e3 * t3608 * t9305 - 0.35089341735807877242e1 * t1003 * t11567 * t2575;
    (t31704, t31706, t31709, t31718)
}
