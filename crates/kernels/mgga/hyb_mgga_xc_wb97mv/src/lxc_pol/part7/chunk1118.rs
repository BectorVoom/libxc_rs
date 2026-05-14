//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1118/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1118<F: Float>(t4550: F, t522: F, t1137: F, t1127: F, t1144: F, t1158: F, t1161: F, t11766: F, t11978: F, t11982: F, t11985: F, t11988: F, t12018: F, t12021: F, t12024: F, t12029: F, t12038: F, t12041: F, t12046: F, t12050: F, t2900: F, t2957: F, t3724: F, t3784: F, t3785: F, t3788: F, t3823: F, t3826: F, t3829: F, t4620: F, t7908: F, t9764: F) -> (F, F, F) {
    let t12053 = t522 * t4550;
    let t12054 = t1137 * t12053;
    let t12057 = -0.1008e1 * t7908 * t11978 - 0.336e0 * t2957 * t11982 + 0.176e0 * t1158 * t11985 - 0.176e0 * t1161 * t11988 - 0.24e-1 * t9764 * t4620 + 0.48e-1 * t1161 * t12018 - 0.768e-3 * t1158 * t12021 + 0.768e-3 * t1161 * t12024 + 0.48e-4 * t11766 * t1144 + 800.0 / 9.0 * t3784 * t12029 + 800.0 / 9.0 * t3788 * t12029 + 800.0 / 27.0 * t3826 * t12029 + 800.0 / 27.0 * t3829 * t12029 + 100.0 / 9.0 * t3724 * t12038 - 100.0 / 9.0 * t12041 * t3785 + 400.0 / 27.0 * t3823 * t12029 - 200.0 / 9.0 * t12046 * t3785 - 0.12e-1 * t2900 * t12050 - 0.16e-1 * t1127 * t12054;
    (t12053, t12054, t12057)
}
