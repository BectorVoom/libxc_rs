//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1359/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1359<F: Float>(t396: F, t513: F, t1126: F, t32776: F, t516: F, t10069: F, t10087: F, t10133: F, t11794: F, t11821: F, t16026: F, t32670: F, t32676: F, t32681: F, t32848: F, t32851: F, t33041: F, t33046: F, t33067: F, t33074: F, t33077: F, t33392: F, t3685: F, t3732: F, t7848: F, t7899: F, t7903: F, t9826: F, t9831: F, t9839: F, t9890: F, t9893: F) -> (F,) {
    let t33413 = t513 * t396;
    let t33414 = t1126 * t33413;
    let t33417 = t516 * t32776;
    let t33436 = 0.18773333333333333333e-2 * t9893 * t32848 - 0.18773333333333333333e-2 * t9826 * t32851 + 0.576e-2 * t16026 * t11794 * t9831 - 0.864e-2 * t10087 * t11794 * t7899 + 0.12096e-1 * t10069 * t33392 - 0.47407407407407407408e0 * t9890 * t32670 - 0.12288e-4 * t33414 * t32676 + 0.12288e-4 * t33417 * t32681 + 0.144e-2 * t7848 * t11821 * t9831 - 0.1728e-2 * t7903 * t33041 - 0.14222222222222222222e1 * t9839 * t32670 - 0.1728e-2 * t7903 * t33046 + 0.51626666666666666667e-5 * t3685 * t33067 + 0.576e-2 * t16026 * t3732 * t33077 + 0.576e-3 * t10133 * t33074;
    (t33436,)
}
