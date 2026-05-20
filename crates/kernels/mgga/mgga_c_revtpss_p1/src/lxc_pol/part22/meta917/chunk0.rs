//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3126/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3126<F: Float>(t342: F, t378: F, t43536: F, t11631: F, t43350: F, t16558: F, t989: F, t1071: F, t12166: F, t12077: F, t43346: F, t42872: F) -> (F, F, F, F, F, F, F) {
    let t55569 = t342 * t43536 * t378;
    let t55570 = t43350 * t11631;
    let t55575 = t989 * t16558;
    let t55579 = t342 * t12166 * t1071;
    let t55583 = t342 * t12077 * t1071;
    let t55593 = t342 * t43346 * t378;
    let t55594 = t43350 * t42872;
    (t55569, t55570, t55575, t55579, t55583, t55593, t55594)
}
