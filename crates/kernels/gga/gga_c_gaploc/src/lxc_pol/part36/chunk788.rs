//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 788/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk788<F: Float>(t2268: F, t26938: F, t3133: F, t31591: F, t4261: F, t9074: F, t39731: F, t2321: F, t34600: F, t1064: F, t2343: F, t42009: F, t42299: F, t42687: F, t42689: F, t42691: F, t42694: F, t42695: F, t42698: F, t42700: F, t42703: F, t42706: F, t42708: F, t42709: F, t42712: F) -> (F,) {
    let t42715 = 0.34146007962811379518e0 * t2268 * t26938 * t3133;
    let t42717 = t9074 * t4261 * t31591;
    let t42718 = 0.47425011059460249332e-2 * t42717;
    let t42719 = 0.23712505529730124666e-2 * t39731;
    let t42721 = t9074 * t34600 * t2321;
    let t42722 = 0.23712505529730124666e-2 * t42721;
    let t42723 = -0.85365019907028448797e-1 * t2268 * t1064 * t42299 + 0.56910013271352299198e-1 * t2268 * t2343 * t42009 + t42687 - t42689 - t42691 - t42694 - 0.2276400530854091968e0 * t42695 + t42698 - 0.1707300398140568976e0 * t42700 + t42703 + t42706 + t42708 + 0.7588001769513639893e-1 * t42709 + t42712 + t42715 + t42718 + t42719 + t42722;
    (t42723,)
}
