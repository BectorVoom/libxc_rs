//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 907/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk907<F: Float>(t2321: F, t34600: F, t9074: F, t1064: F, t2268: F, t2343: F, t42009: F, t42299: F, t42687: F, t42689: F, t42691: F, t42694: F, t42695: F, t42698: F, t42700: F, t42703: F, t42706: F, t42708: F, t42709: F, t42712: F, t42715: F, t42718: F, t42719: F) -> F {
    let t42721 = t9074 * t34600 * t2321;
    let t42722 = F::cast_from(0.23712505529730124666e-2_f64) * t42721;
    let t42723 = -F::cast_from(0.85365019907028448797e-1_f64) * t2268 * t1064 * t42299 + F::cast_from(0.56910013271352299198e-1_f64) * t2268 * t2343 * t42009 + t42687 - t42689 - t42691 - t42694 - F::cast_from(0.2276400530854091968e0_f64) * t42695 + t42698 - F::cast_from(0.1707300398140568976e0_f64) * t42700 + t42703 + t42706 + t42708 + F::cast_from(0.7588001769513639893e-1_f64) * t42709 + t42712 + t42715 + t42718 + t42719 + t42722;
    t42723
}
