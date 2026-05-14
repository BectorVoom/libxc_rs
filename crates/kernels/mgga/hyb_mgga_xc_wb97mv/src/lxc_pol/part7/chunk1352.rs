//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1352/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1352<F: Float>(t11780: F, t2869: F, t4083: F, t4951: F, t9896: F, t10039: F, t10054: F, t10133: F, t11834: F, t11837: F, t11854: F, t11923: F, t27964: F, t2817: F, t2828: F, t2832: F, t2839: F, t32742: F, t32747: F, t32757: F, t32937: F, t32941: F, t33085: F, t33088: F, t3746: F, t3760: F, t4077: F, t535: F, t7848: F, t9737: F) -> (F,) {
    let t33103 = t11780 * t4083 * t2869;
    let t33119 = t9896 * t4951;
    let t33134 = -0.96e-1 * t11923 * t32937 + 0.11264e-4 * t3760 * t32941 + 0.768e-6 * t2817 * t33103 + 0.2304e-5 * t2828 * t33103 - 0.2304e-5 * t2832 * t32742 - 0.98304e-7 * t535 * t27964 * t32747 + 0.72e-1 * t10133 * t32757 - 11200.0 / 9.0 * t11834 * t9737 - 1600.0 / 27.0 * t11837 * t9737 - 3200.0 / 27.0 * t10039 * t33119 - 0.264e-2 * t7848 * t3746 * t4077 * t2839 - 3200.0 / 3.0 * t10054 * t33119 + 3200.0 / 3.0 * t11854 * t9737 - 0.528e-3 * t2828 * t33085 + 0.528e-3 * t2832 * t33088;
    (t33134,)
}
