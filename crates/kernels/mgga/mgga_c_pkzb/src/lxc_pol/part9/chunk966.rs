//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 966/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk966<F: Float>(t7550: F, t7574: F, t301: F, t761: F, t758: F, t1125: F, t5939: F, t757: F, t2096: F, t2908: F, t2886: F, t434: F) -> (F, F, F, F, F, F, F) {
    let t7575 = t7550 + t7574;
    let t7577 = t301 * t7575 * t761;
    let t7578 = t758 * t7577;
    let t7581 = t5939 * t1125;
    let t7582 = t757 * t7581;
    let t7585 = F::cast_from(0.15244095330869239812e-2_f64) * t2096 * t2908;
    let t7586 = t434 * t2886;
    (t7575, t7577, t7578, t7581, t7582, t7585, t7586)
}
