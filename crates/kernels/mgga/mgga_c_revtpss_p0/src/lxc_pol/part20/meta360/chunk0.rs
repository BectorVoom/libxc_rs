//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1309/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1309<F: Float>(t10073: F, t10934: F, t253: F, t39552: F, t2783: F, t9646: F, t22: F, t251: F, t837: F, t2722: F, t860: F, t231: F, t2782: F) -> (F, F, F, F, F) {
    let t39694 = t10073 * t10934;
    let t39697 = F::cast_from(0.88356352675825229576e-3_f64) * t39552 * t253;
    let t39698 = t9646 * t2783;
    let t39701 = t39698 * t251 * t22 * t837;
    let t39704 = t860 * t2722;
    let t39707 = t2782 * t2783 * t39704 * t231;
    (t39694, t39697, t39701, t39704, t39707)
}
