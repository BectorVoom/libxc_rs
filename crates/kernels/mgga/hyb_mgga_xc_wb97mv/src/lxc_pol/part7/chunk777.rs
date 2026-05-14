//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 777/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk777<F: Float>(t7: F, t132: F, t1319: F, t1376: F, t220: F, t291: F, t4143: F, t4267: F, t3988: F, t2456: F, t3979: F, t926: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F) {
    let t8 = t7 <= zeta_threshold;
    let t9 = rho0 <= dens_threshold || t8;
    let t133 = t132 <= zeta_threshold;
    let t4271 = piecewise3(t9, 0.0, t4143 * t291 / 2.0 + t1319 * t1376 + t220 * t4267 / 2.0);
    let t4273 = piecewise3(t133, 0.0, t3988);
    let t4283 = piecewise3(t133, 0.0, 4.0 / 9.0 * t2456 * t3979 - t926 * t3988 / 3.0);
    (t4271, t4273, t4283)
}
