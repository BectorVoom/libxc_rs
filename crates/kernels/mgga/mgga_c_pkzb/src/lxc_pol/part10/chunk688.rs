//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 688/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk688<F: Float>(t12: F, t1430: F, t1151: F, t1153: F, t3000: F, t318: F, t319: F, t808: F, t810: F, t201: F, t1167: F, t204: F, t648: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F) {
    let t84 = t12 <= zeta_threshold;
    let t203 = rho0 <= dens_threshold || t84;
    let t3004 = 2.0 * t1430;
    let t3005 = piecewise3(t84, 0.0, t3004);
    let t3009 = piecewise3(t203, 0.0, t1151 * t810 / 2.0 + t808 * t1153 / 2.0 + t3000 * t319 / 2.0 + t318 * t3005 / 2.0);
    let t3010 = t201 * t3009;
    let t3017 = t204 * t648 * t1167;
    (t3004, t3005, t3010, t3017)
}
