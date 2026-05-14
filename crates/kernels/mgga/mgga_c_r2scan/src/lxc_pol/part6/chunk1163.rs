//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1163/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1163<F: Float>(t424: F, t595: F, t21211: F, t40: F, t5243: F, t171: F, t1871: F, t591: F, t5249: F, t1981: F, t5461: F, t5456: F, t5460: F, t5593: F, t1691: F, t1719: F) -> (F, F, F, F, F, F, F) {
    let t21217 = t595 * t424;
    let t21219 = t21211 * t5243 * t40;
    let t21221 = 0.31847245507892832661e-2 * t21217 * t21219;
    let t21224 = t171 * t591 * t1871;
    let t21225 = t5249 * t424 * t21224;
    let t21228 = t424 * t1981 * t5461;
    let t21232 = 0.12304822629859687989e5 * t5460 * t5593 * t5456;
    let t21234 = t1691 * t1719;
    (t21217, t21221, t21224, t21225, t21228, t21232, t21234)
}
