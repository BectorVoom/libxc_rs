//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 764/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk764<F: Float>(t7: F, t132: F, t1325: F, t1382: F, t220: F, t291: F, t4094: F, t4218: F, t3938: F, t2460: F, t3925: F, t937: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F) {
    let t8 = t7 <= zeta_threshold;
    let t9 = rho0 <= dens_threshold || t8;
    let t133 = t132 <= zeta_threshold;
    let t4222 = piecewise3(t9, 0.0, t4094 * t291 / 2.0 + t1325 * t1382 + t220 * t4218 / 2.0);
    let t4224 = piecewise3(t133, 0.0, t3938);
    let t4234 = piecewise3(t133, 0.0, 4.0 / 9.0 * t2460 * t3925 - t937 * t3938 / 3.0);
    (t4222, t4224, t4234)
}
