//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 789/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk789<F: Float>(t12770: F, t2312: F, t10590: F, t2321: F, t882: F, t10156: F, t1063: F, t6750: F, t2268: F, t2854: F, t29984: F, t6320: F, t41838: F, t426: F, t535: F, t2765: F, t31300: F) -> (F, F, F, F, F, F) {
    let t42745 = t2312 * t12770;
    let t42748 = t882 * t10590 * t2321;
    let t42751 = t1063 * t10156 * t6750;
    let t42756 = 0.34146007962811379518e0 * t2268 * t6320 * t2854 * t29984;
    let t42759 = t2268 * t535 * t41838 * t426;
    let t42763 = 0.85365019907028448797e-1 * t1063 * t2765 * t31300;
    (t42745, t42748, t42751, t42756, t42759, t42763)
}
