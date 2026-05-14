//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1371/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1371<F: Float>(t3728: F, t9825: F, t7837: F, t11689: F, t9897: F, t10079: F, t10141: F, t10147: F, t10150: F, t10156: F, t10166: F, t10172: F, t10182: F, t10190: F, t11801: F, t11886: F, t2853: F, t2857: F, t32595: F, t32643: F, t32824: F, t33139: F, t33475: F, t4608: F, t4636: F, t9850: F) -> (F, F) {
    let t33881 = t3728 * t9825;
    let t33886 = t3728 * t7837;
    let t33901 = t11689 * t9897;
    let t33914 = 0.128e0 * t33881 * t11886 - 0.2112e1 * t10079 * t32643 + 0.144e0 * t33886 * t11801 + 0.14222222222222222222e-2 * t9850 * t32824 + 0.42666666666666666667e-2 * t10147 * t32824 + 0.42666666666666666667e-2 * t10182 * t32824 + 2.0 * t4608 * t2853 + 12.0 * t4636 * t2857 - 0.53333333333333333333e0 * t10141 * t33139 + 0.26666666666666666667e1 * t10150 * t33901 - 0.32e1 * t10156 * t33139 + 0.32e1 * t10156 * t33901 - 0.5632e-5 * t10172 * t32595 + 0.16896e-4 * t10190 * t33475 - 0.35555555555555555555e0 * t10166 * t33139;
    (t33901, t33914)
}
