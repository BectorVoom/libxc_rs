//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1067/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1067<F: Float>(t4246: F, t846: F, t4243: F, t10921: F, t10923: F, t10926: F, t10929: F, t10932: F, t10935: F, t10939: F, t10942: F, t10946: F, t2301: F, t3416: F, t3436: F, t4230: F, t6929: F, t6982: F) -> (F, F, F) {
    let t11123 = t4246 * t846;
    let t11126 = t4243 * t846;
    let t11129 = 0.11696447245269292414e1 * t3416 * t3436 - 0.11696447245269292414e1 * t6929 * t4230 + t10921 - t10923 - t10926 + t10929 + t10932 + t10935 - t10939 - t10942 - t10946 - 0.10389515463408878255e3 * t6982 * t11123 - 0.11696447245269292414e1 * t2301 * t11126;
    (t11123, t11126, t11129)
}
