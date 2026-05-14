//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1351/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1351<F: Float>(t10069: F, t10084: F, t10087: F, t11876: F, t16064: F, t2817: F, t2823: F, t2828: F, t2832: F, t32926: F, t32931: F, t32934: F, t33055: F, t33063: F, t33067: F, t33074: F, t33077: F, t33082: F, t33085: F, t33088: F, t3736: F, t3771: F, t7818: F, t7838: F, t9898: F) -> (F,) {
    let t33097 = -0.71111111111111111112e0 * t2828 * t33055 - 0.71111111111111111112e0 * t2832 * t33055 + 0.17777777777777777778e0 * t11876 * t9898 - 0.16128e-1 * t16064 * t33063 + 0.51626666666666666667e-5 * t3771 * t33067 - 0.23703703703703703704e0 * t2817 * t33055 - 0.23703703703703703704e0 * t2823 * t33055 + 0.12096e-1 * t10069 * t33074 - 0.864e-2 * t10087 * t3736 * t33077 - 0.3696e-2 * t7818 * t33082 - 0.176e-3 * t2817 * t33085 + 0.176e-3 * t2823 * t33088 + 0.64e-1 * t10084 * t32926 + 0.2304e-5 * t7838 * t32931 + 0.64e-1 * t10084 * t32934;
    (t33097,)
}
