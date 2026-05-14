//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1150/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1150<F: Float>(t12: F, t28885: F, t11118: F, t11125: F, t1151: F, t1153: F, t3000: F, t3005: F, t30990: F, t30991: F, t30993: F, t30998: F, t31004: F, t31005: F, t31007: F, t31017: F, t318: F, t319: F, t3706: F, t3710: F, t808: F, t810: F, t9729: F, t9738: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F,) {
    let t84 = t12 <= zeta_threshold;
    let t203 = rho0 <= dens_threshold || t84;
    let t31035 = piecewise3(t84, 0.0, t28885);
    let t31039 = piecewise3(t203, 0.0, (t30990 + t30991 + t30993 + t30998 + t31004 + t31005 + t31007 + t31017) * t319 / 2.0 + t11118 * t810 / 2.0 + 3.0 / 2.0 * t9729 * t1153 + 3.0 / 2.0 * t3706 * t3005 + 3.0 / 2.0 * t3000 * t3710 + 3.0 / 2.0 * t1151 * t9738 + t808 * t11125 / 2.0 + t318 * t31035 / 2.0);
    (t31039,)
}
