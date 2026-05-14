//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 992/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk992<F: Float>(t2895: F, t3732: F, t3736: F, t1519: F, t2869: F, t1522: F, t1128: F, t2873: F, t1115: F, t1514: F, t1127: F, t1132: F, t1158: F, t1161: F, t1541: F, t2900: F, t2915: F, t2946: F, t2953: F, t2957: F, t3724: F, t3729: F, t3800: F, t3823: F, t3826: F, t3829: F, t7913: F, t8089: F, t9737: F, t9747: F, t9752: F, t9755: F, t9768: F) -> (F, F, F, F) {
    let t9774 = t2895 * t3732;
    let t9777 = t2895 * t3736;
    let t9784 = t1519 * t2869;
    let t9792 = t1522 * t2869;
    let t9793 = t1128 * t9792;
    let t9796 = t1522 * t2873;
    let t9797 = t1128 * t9796;
    let t9806 = t1514 * t1115;
    let t9819 = -0.32e-1 * t3729 * t3800 - 0.256e-3 * t1127 * t9774 + 0.256e-3 * t1132 * t9777 - 0.16e-1 * t1127 * t9752 + 0.16e-1 * t1132 * t9755 + 0.9e-1 * t2953 * t1128 * t9784 - 0.108e0 * t2915 * t9768 - 0.768e-3 * t1158 * t9774 - 0.108e0 * t2915 * t9793 + 0.126e0 * t2957 * t9797 + 0.768e-3 * t1161 * t9777 - 0.12e-1 * t2900 * t9793 + 0.18e-1 * t2946 * t9797 + 100.0 / 9.0 * t3724 * t9806 + 0.48e-4 * t7913 * t1541 + 400.0 / 27.0 * t3823 * t9737 + 800.0 / 27.0 * t3826 * t9737 + 800.0 / 27.0 * t3829 * t9737 + 0.36e-1 * t8089 * t9747;
    (t9784, t9792, t9796, t9819)
}
