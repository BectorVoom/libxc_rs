//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1339/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1339<F: Float>(t2895: F, t32718: F, t2869: F, t3746: F, t4077: F, t10143: F, t10152: F, t1148: F, t11867: F, t27976: F, t28087: F, t2817: F, t2823: F, t2828: F, t2832: F, t28488: F, t28500: F, t28563: F, t32688: F, t32692: F, t32695: F, t32698: F, t32702: F, t32711: F, t32715: F, t3678: F, t4523: F, t9843: F, t9887: F, t9954: F) -> (F, F) {
    let t32719 = t32718 * t2895;
    let t32729 = t3746 * t4077 * t2869;
    let t32732 = 0.17777777777777777778e0 * t9887 * t11867 + 0.35555555555555555556e0 * t32688 * t9843 - 800.0 / 3.0 * t28488 * t32692 + 800.0 / 3.0 * t32695 * t10143 + 4000.0 * t28563 * t3678 * t32698 - 4000.0 * t32702 * t10152 - 5600.0 * t28500 * t32692 + 5600.0 * t1148 * t9954 * t10143 + 0.12288e-4 * t27976 * t32711 + 0.768e-6 * t2817 * t32715 - 0.768e-6 * t2823 * t32719 + 0.2304e-5 * t2828 * t32715 - 0.2304e-5 * t2832 * t32719 + 0.96e-4 * t28087 * t4523 - 0.176e-3 * t2817 * t32729;
    (t32729, t32732)
}
