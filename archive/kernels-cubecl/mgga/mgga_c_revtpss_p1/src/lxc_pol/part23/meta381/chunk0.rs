//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1722/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1722<F: Float>(t1071: F, t3316: F, t342: F, t1647: F, t3298: F, t4980: F, t989: F, t4995: F, t1086: F, t1678: F, t994: F) -> (F, F, F, F, F, F, F) {
    let t16505 = t3316 * t1071;
    let t16506 = t342 * t16505;
    let t16509 = t1647 * t3298;
    let t16520 = t989 * t4980;
    let t16523 = t989 * t4995;
    let t16543 = t1086 * t1678;
    let t16544 = t994 * t16543;
    (t16505, t16506, t16509, t16520, t16523, t16543, t16544)
}
