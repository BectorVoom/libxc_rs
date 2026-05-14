//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 728/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk728<F: Float>(t1426: F, t2085: F, t535: F, t598: F, t537: F, t7605: F, t1576: F, t2001: F, t1581: F, t542: F, t1588: F, t1988: F, t2327: F, t1487: F, t6: F, t422: F, t599: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t8838 = t1426 * t535 * t2085;
    let t8839 = t598 * t8838;
    let t8841 = t7605 * t537;
    let t8843 = t2001 * t1576;
    let t8845 = t2001 * t1581;
    let t8847 = t7605 * t542;
    let t8849 = t2001 * t1588;
    let t8851 = t1988 * t2327;
    let t8853 = t6 * t1487;
    let t8855 = t422 * t8853 * t599;
    (t8838, t8839, t8841, t8843, t8845, t8847, t8849, t8851, t8853, t8855)
}
