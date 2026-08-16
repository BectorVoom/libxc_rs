//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 908/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk908(t2321: f64, t34600: f64, t9074: f64, t1064: f64, t2268: f64, t2343: f64, t42009: f64, t42299: f64, t42687: f64, t42689: f64, t42691: f64, t42694: f64, t42695: f64, t42698: f64, t42700: f64, t42703: f64, t42706: f64, t42708: f64, t42709: f64, t42712: f64, t42715: f64, t42718: f64, t42719: f64) -> f64 {
    let t42721 = t9074 * t34600 * t2321;
    let t42722 = 0.23712505529730124666e-2_f64 * t42721;
    let t42723 = -0.85365019907028448797e-1_f64 * t2268 * t1064 * t42299 + 0.56910013271352299198e-1_f64 * t2268 * t2343 * t42009 + t42687 - t42689 - t42691 - t42694 - 0.2276400530854091968e0_f64 * t42695 + t42698 - 0.1707300398140568976e0_f64 * t42700 + t42703 + t42706 + t42708 + 0.7588001769513639893e-1_f64 * t42709 + t42712 + t42715 + t42718 + t42719 + t42722;
    t42723
}
