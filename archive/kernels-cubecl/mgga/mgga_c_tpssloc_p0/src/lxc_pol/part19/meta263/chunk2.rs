//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1014/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1014<F: Float>(t11153: F, t3439: F, t9288: F, t974: F, t11147: F, t11545: F, t11660: F, t1216: F, t4582: F, t10913: F, t4987: F, t3247: F, t415: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11759 = t3439 * t11153;
    let t11760 = t11759 * t9288;
    let t11761 = t974 * t11760;
    let t11764 = t11545 * t11147;
    let t11765 = t11764 * t9288;
    let t11766 = t974 * t11765;
    let t11769 = t11660 * t1216;
    let t11770 = t4582 * t11769;
    let t11773 = t4987 * t10913;
    let t11774 = t4582 * t11773;
    let t11778 = F::cast_from(1.0_f64) / t415 / t3247;
    (t11760, t11761, t11765, t11766, t11769, t11770, t11773, t11774, t11778)
}
