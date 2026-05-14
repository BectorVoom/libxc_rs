//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 241/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk241<F: Float>(t143: F, t172: F, t187: F, t693: F, t707: F, t711: F, t715: F, t719: F, t723: F, t727: F, t731: F, t735: F, t739: F, t758: F) -> (F,) {
    let t144 = 0.135e1 <= t143;
    let t762 = piecewise3(t144, -t693 * t707 / 18.0 + t711 * t707 / 240.0 - t715 * t707 / 4480.0 + t719 * t707 / 103680.0 - t723 * t707 / 2838528.0 + t727 * t707 / 89456640.0 - t731 * t707 / 0.31850496e10 + t735 * t707 / 0.1263403008e12, -8.0 / 3.0 * t172 * t758 - 8.0 / 3.0 * t739 * t187);
    (t762,)
}
