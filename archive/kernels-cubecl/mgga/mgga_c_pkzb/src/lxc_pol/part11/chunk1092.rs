//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1092/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1092<F: Float>(t4794: F, t7: F, t1448: F, t448: F, t34: F, t38: F, t4810: F, t2620: F, t5322: F, t1532: F, t2557: F, t49: F) -> (F, F, F, F, F, F) {
    let t19396 = t7 * t4794;
    let t19467 = t448 * t1448;
    let t19523 = t34 * t4794;
    let t19530 = t38 * t4810;
    let t19620 = t2620 * t5322;
    let t19623 = t2557 * t49 * t1532;
    (t19396, t19467, t19523, t19530, t19620, t19623)
}
