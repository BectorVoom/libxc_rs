//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 805/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk805<F: Float>(t598: F, t8838: F, t537: F, t7605: F, t1576: F, t2001: F, t1581: F, t542: F, t1588: F, t1988: F, t2327: F, t1487: F, t6: F) -> (F, F, F, F, F, F, F, F) {
    let t8839 = t598 * t8838;
    let t8841 = t7605 * t537;
    let t8843 = t2001 * t1576;
    let t8845 = t2001 * t1581;
    let t8847 = t7605 * t542;
    let t8849 = t2001 * t1588;
    let t8851 = t1988 * t2327;
    let t8853 = t6 * t1487;
    (t8839, t8841, t8843, t8845, t8847, t8849, t8851, t8853)
}
