//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2634/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2634<F: Float>(t5154: F, t9722: F, t2221: F, t5166: F, t1788: F, t9216: F, t9218: F, t9494: F, t15892: F, t2535: F, t2528: F, t15971: F, t588: F) -> (F, F, F, F, F, F, F, F) {
    let t54451 = t5154 * t9722;
    let t54456 = t2221 * t5166;
    let t54460 = t9216 * t1788;
    let t54462 = t9218 * t1788;
    let t54467 = t5154 * t9494;
    let t54469 = t15892 * t2535;
    let t54471 = t15892 * t2528;
    let t54477 = t588 * t15971;
    (t54451, t54456, t54460, t54462, t54467, t54469, t54471, t54477)
}
