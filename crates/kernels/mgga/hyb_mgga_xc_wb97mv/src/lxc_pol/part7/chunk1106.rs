//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1106/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1106<F: Float>(t1111: F, t4083: F, t11780: F, t1114: F, t516: F, t9878: F, t2893: F, t646: F, t4558: F, t1142: F, t4541: F, t3799: F, t3809: F, t3813: F, t4619: F, t7833: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11781 = t4083 * t1111;
    let t11782 = t11780 * t11781;
    let t11786 = t11780 * t4083 * t1114;
    let t11789 = t516 * t9878;
    let t11790 = t646 * t2893;
    let t11791 = t11790 * t4558;
    let t11794 = t1142 * t4541;
    let t11798 = t3799 * t3809;
    let t11801 = t3799 * t3813;
    let t11804 = t7833 * t4619;
    (t11781, t11782, t11786, t11789, t11791, t11794, t11798, t11801, t11804)
}
