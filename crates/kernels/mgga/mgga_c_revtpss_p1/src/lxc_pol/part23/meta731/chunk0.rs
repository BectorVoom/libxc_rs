//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2500/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2500<F: Float>(t1531: F, t36: F, t14362: F, t9863: F, t9866: F, t2609: F, t4395: F, t10115: F, t1570: F, t11007: F, t1579: F, t4322: F, t9292: F) -> (F, F, F, F, F, F, F) {
    let t50089 = t36 * t1531;
    let t50092 = t14362 * t9863;
    let t50094 = t14362 * t9866;
    let t50097 = t4395 * t2609;
    let t50098 = F::new(3.0) * t50097;
    let t50155 = t10115 * t1570;
    let t50161 = t11007 * t1579;
    let t50166 = t9292 * t4322;
    (t50089, t50092, t50094, t50098, t50155, t50161, t50166)
}
