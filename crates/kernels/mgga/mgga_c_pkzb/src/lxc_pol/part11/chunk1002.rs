//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1002/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1002<F: Float>(t2463: F, t23: F, t4810: F, t4794: F, t7: F, t1448: F, t448: F, t34: F, t38: F, t2620: F, t5322: F, t1532: F, t2557: F, t49: F, t4865: F, t7046: F) -> (F, F, F, F, F, F, F, F, F) {
    let t19338 = t2463 * t2463;
    let t19339 = 1.0 / t19338;
    let t19377 = t23 * t4810;
    let t19396 = t7 * t4794;
    let t19467 = t448 * t1448;
    let t19523 = t34 * t4794;
    let t19530 = t38 * t4810;
    let t19620 = t2620 * t5322;
    let t19623 = t2557 * t49 * t1532;
    let t19624 = 0.32530743900905219526e-1 * t19623;
    let t19625 = t7046 * t4865;
    (t19339, t19377, t19396, t19467, t19523, t19530, t19620, t19624, t19625)
}
