//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1341/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1341<F: Float>(t12058: F, t7833: F, t12062: F, t1122: F, t396: F, t1126: F, t14633: F, t2901: F, t10065: F, t10069: F, t10087: F, t10133: F, t24260: F, t24273: F, t28161: F, t2823: F, t2828: F, t2832: F, t28748: F, t2895: F, t32655: F, t32658: F, t32729: F, t32734: F, t32742: F, t32747: F, t32750: F, t32757: F, t32760: F, t32767: F, t3799: F, t516: F, t7832: F, t7848: F, t9792: F) -> (F, F, F, F, F) {
    let t32770 = t7833 * t12058;
    let t32773 = t7833 * t12062;
    let t32776 = t1122 * t396;
    let t32777 = t1126 * t32776;
    let t32778 = t14633 * t2901;
    let t32781 = 0.176e-3 * t2823 * t32734 - 0.528e-3 * t2828 * t32729 + 0.528e-3 * t2832 * t32734 - 0.768e-6 * t2823 * t32742 - 0.98304e-7 * t516 * t28748 * t32747 + 0.1152e-4 * t7848 * t32750 * t2895 - 0.108e1 * t10087 * t3799 * t9792 + 0.1512e1 * t10069 * t32757 + 0.9216e-2 * t24273 * t32760 - 0.288e0 * t28161 * t32655 - 0.192e0 * t24260 * t32658 + 0.72e-1 * t10133 * t32767 + 0.64e-1 * t7832 * t32770 - 0.96e-1 * t10065 * t32773 + 0.1024e-2 * t32777 * t32778;
    (t32770, t32773, t32776, t32778, t32781)
}
