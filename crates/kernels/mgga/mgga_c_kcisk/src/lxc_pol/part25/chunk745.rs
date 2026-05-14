//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 745/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk745<F: Float>(t1957: F, t9696: F, t2799: F, t5213: F, t5218: F, t1930: F, t736: F, t1934: F, t654: F) -> (F, F, F, F, F, F) {
    let t9697 = t9696 * t1957;
    let t9698 = t5213 * t2799;
    let t9699 = t2799 * t1957;
    let t9701 = 2.0 * t5218 * t9699;
    let t9702 = t1930 * t736;
    let t9704 = t1934 * t654;
    (t9697, t9698, t9699, t9701, t9702, t9704)
}
