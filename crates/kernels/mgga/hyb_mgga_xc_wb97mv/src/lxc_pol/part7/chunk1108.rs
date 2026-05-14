//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1108/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1108<F: Float>(t1137: F, t11821: F, t10069: F, t10087: F, t1138: F, t11766: F, t11769: F, t11772: F, t11775: F, t11782: F, t11786: F, t11789: F, t11791: F, t11794: F, t11798: F, t11801: F, t11804: F, t11810: F, t11814: F, t2817: F, t2823: F, t2828: F, t2832: F, t3724: F, t4523: F, t7848: F, t7854: F, t9887: F, t9977: F, t9981: F, t9984: F) -> (F, F) {
    let t11822 = t11821 * t1137;
    let t11825 = -0.16e-1 * t11766 * t1138 + 400.0 / 27.0 * t3724 * t11769 + 24.0 * t9981 * t11772 - 360.0 * t9984 * t11775 + 504.0 * t9977 * t11772 + 0.768e-6 * t2817 * t11782 - 0.768e-6 * t2823 * t11786 - 0.3072e-5 * t11789 * t11791 + 0.72e-3 * t7848 * t11794 * t1137 - 0.108e1 * t10087 * t11798 + 0.1512e1 * t10069 * t11801 + 0.576e0 * t7854 * t11804 + 0.96e-4 * t9887 * t4523 - 0.176e-3 * t2817 * t11810 + 0.176e-3 * t2823 * t11814 - 0.528e-3 * t2828 * t11810 + 0.528e-3 * t2832 * t11814 + 0.48e-4 * t2817 * t11822;
    (t11822, t11825)
}
