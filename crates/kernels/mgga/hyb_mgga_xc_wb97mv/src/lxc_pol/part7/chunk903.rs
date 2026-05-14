//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 903/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk903<F: Float>(t2968: F, t8195: F, t1175: F, t2004: F, t3131: F, t549: F, t19: F, t3135: F, t1183: F, t6134: F, t1852: F, t3028: F, t3034: F, t25: F, t456: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t8196 = t8195 * t2968;
    let t8198 = t1175 * t2004;
    let t8206 = t549 * t3131;
    let t8208 = t19 * t8206 / 32.0;
    let t8209 = t549 * t3135;
    let t8211 = t19 * t8209 / 32.0;
    let t8216 = t6134 * t1183;
    let t8219 = 2.0 / 243.0 * t1852 * t3028;
    let t8221 = 2.0 / 81.0 * t1852 * t3034;
    let t8223 = 1.0 / t25 / t456;
    (t8196, t8198, t8206, t8208, t8209, t8211, t8216, t8219, t8221, t8223)
}
