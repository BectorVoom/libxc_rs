//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1404/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1404<F: Float>(t11818: F, t1216: F, t248: F, t1213: F, t11552: F, t221: F, t456: F, t1197: F, t698: F, t1174: F, t1176: F, t3242: F) -> (F, F, F, F, F, F, F) {
    let t11820 = t248 * t11818 * t1216;
    let t11821 = t1213 * t11820;
    let t11832 = t221 * t11552;
    let t11834 = F::cast_from(5.0_f64) / F::cast_from(1296.0_f64) * t456 * t11832;
    let t11835 = t698 * t1197;
    let t11836 = t1174 * t11835;
    let t11848 = t1176 * t3242;
    (t11820, t11821, t11832, t11834, t11835, t11836, t11848)
}
