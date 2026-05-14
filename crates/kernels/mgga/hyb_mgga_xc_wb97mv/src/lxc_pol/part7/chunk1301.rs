//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1301/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1301<F: Float>(t23180: F, t23183: F, t23192: F, t27021: F, t27024: F, t27027: F, t31779: F, t31782: F, t31810: F, t939: F, t1386: F, t238: F, t242: F, t9290: F, t11313: F, t800: F) -> (F, F, F, F) {
    let t31811 = t23192 - 56.0 / 27.0 * t23180 + 4.0 / 9.0 * t23183 - 56.0 / 27.0 * t27021 + 16.0 / 9.0 * t27024 - 2.0 / 3.0 * t27027 + 4.0 / 9.0 * t31779 - 2.0 / 3.0 * t31782 + t31810;
    let t31812 = t939 * t31811;
    let t31817 = t238 * t242 * t1386 * t9290;
    let t31820 = t238 * t800 * t11313;
    (t31811, t31812, t31817, t31820)
}
