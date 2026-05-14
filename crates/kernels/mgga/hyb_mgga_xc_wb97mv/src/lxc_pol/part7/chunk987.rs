//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 987/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk987<F: Float>(t132: F, t1232: F, t7721: F, t2799: F, t3: F, t1874: F, t2013: F, t2039: F, t339: F, t3649: F, t3652: F, t457: F, t9280: F, t259: F, t9669: F, zeta_threshold: F) -> (F, F) {
    let t133 = t132 <= zeta_threshold;
    let t9670 = t7721 * t1232;
    let t9673 = t2799 * t3;
    let t9683 = piecewise3(t133, 0.0, -8.0 / 27.0 * t9670 * t2039 - 16.0 / 9.0 * t9673 * t9280 + 4.0 / 9.0 * t3649 * t2013 - 8.0 / 3.0 * t339 * t1874 + 8.0 * t3652 * t457);
    let t9685 = (t9669 + t9683) * t259;
    (t9670, t9685)
}
