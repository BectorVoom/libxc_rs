//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1001/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1001<F: Float>(t1528: F, t2869: F, t2873: F, t1519: F, t2856: F, t2849: F, t1112: F, t1117: F, t2817: F, t2823: F, t2828: F, t2832: F, t3680: F, t3685: F, t3704: F, t3708: F, t3712: F, t3718: F, t3771: F, t3803: F, t505: F, t511: F, t529: F, t9843: F, t9869: F, t9887: F, t9890: F, t9893: F, t9898: F, t9902: F, t9916: F, t9921: F) -> (F, F, F) {
    let t9924 = t1528 * t2869;
    let t9935 = t1528 * t2873;
    let t9940 = t2856 * t1519;
    let t9941 = t9940 * t2849;
    let t9944 = -0.66666666666666666666e-1 * t9887 * t3680 + 0.17777777777777777778e0 * t9890 * t9843 - 0.512e-3 * t9893 * t9869 + 0.26666666666666666667e0 * t2832 * t9898 - 0.1408e-5 * t3771 * t9902 - 0.1408e-5 * t3685 * t9902 + 0.26666666666666666667e0 * t2828 * t9898 + 0.88888888888888888888e-1 * t2817 * t9898 + 0.88888888888888888888e-1 * t2823 * t9898 + 0.9216e-8 * t3771 * t9916 + 0.9216e-8 * t3685 * t9916 - 4.0 * t1117 * t9921 - 4.0 * t1117 * t9924 + 6.0 * t511 * t3718 * t2873 + 42.0 * t529 * t3704 * t2873 + 4.0 * t1112 * t3708 + 2.0 * t505 * t9935 - 8.0 * t3803 * t3712 + 12.0 * t1117 * t9941;
    (t9935, t9940, t9944)
}
