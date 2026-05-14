//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 963/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk963<F: Float>(t132: F, t9274: F, t1232: F, t7198: F, t2456: F, t3: F, t1874: F, t674: F, t2013: F, t2039: F, t3480: F, t3483: F, t457: F, t926: F, t222: F, t37: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t133 = t132 <= zeta_threshold;
    let t9275 = 0.35616666666666666666e-1 * t9274;
    let t9276 = t7198 * t1232;
    let t9279 = t2456 * t3;
    let t9280 = t1874 * t674;
    let t9290 = piecewise3(t133, 0.0, -28.0 / 27.0 * t9276 * t2039 - 16.0 / 9.0 * t9279 * t9280 + 4.0 / 9.0 * t3480 * t2013 + 2.0 / 3.0 * t926 * t1874 - 2.0 * t3483 * t457);
    let t9292 = t222 * t37 * t9290;
    (t9275, t9276, t9280, t9290, t9292)
}
