//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1011/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1011<F: Float>(t10116: F, t2895: F, t1522: F, t2893: F, t10036: F, t10061: F, t10065: F, t10066: F, t10069: F, t10070: F, t10076: F, t10079: F, t10081: F, t10084: F, t10087: F, t10088: F, t10091: F, t10092: F, t10095: F, t10099: F, t10103: F, t10107: F, t10111: F, t2817: F, t2823: F, t2828: F, t2832: F, t3733: F, t3741: F, t3760: F, t7832: F, t7854: F, t9887: F) -> (F, F, F) {
    let t10117 = t10116 * t2895;
    let t10120 = t2893 * t1522;
    let t10121 = t10120 * t2895;
    let t10128 = 0.64e-1 * t7832 * t10061 - 0.96e-1 * t10065 * t10066 + 0.1512e1 * t10069 * t10070 + 0.576e0 * t7854 * t10061 - 0.672e0 * t10076 * t10066 + 0.576e0 * t10079 * t10081 + 0.64e-1 * t10084 * t10081 - 0.108e1 * t10087 * t10088 - 0.48e0 * t10091 * t10092 + 0.576e0 * t7854 * t10095 + 1400.0 / 3.0 * t10099 * t10036 - 0.3072e-5 * t3760 * t10103 + 0.48e-4 * t2817 * t10107 - 0.48e-4 * t2823 * t10111 + 0.96e-4 * t9887 * t3733 + 0.2304e-5 * t2828 * t10117 - 0.2304e-5 * t2832 * t10121 - 0.3072e-5 * t3741 * t10103 + 0.144e-3 * t2828 * t10107;
    (t10117, t10121, t10128)
}
