//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1347/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1347<F: Float>(t12: F, t22148: F, t1151: F, t1153: F, t2159: F, t2163: F, t26784: F, t26790: F, t26797: F, t26802: F, t26805: F, t26817: F, t26819: F, t26820: F, t3000: F, t3005: F, t318: F, t319: F, t3706: F, t3710: F, t7897: F, t7909: F, t808: F, t810: F, t9729: F, t9738: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F,) {
    let t84 = t12 <= zeta_threshold;
    let t203 = rho0 <= dens_threshold || t84;
    let t26836 = piecewise3(t84, 0.0, t22148);
    let t26840 = piecewise3(t203, 0.0, (t26784 + t26790 + t26797 + t26802 + t26805 + t26817 + t26819 + t26820) * t319 / 2.0 + t9729 * t810 + t3706 * t2163 / 2.0 + t7897 * t1153 + 2.0 * t3000 * t3005 + t1151 * t7909 + t2159 * t3710 / 2.0 + t808 * t9738 + t318 * t26836 / 2.0);
    (t26840,)
}
