//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1065/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1065<F: Float>(t1678: F, t3316: F, t342: F, t6299: F, t73: F, t1065: F, t6244: F, t3172: F, t6301: F, t1041: F, t6258: F, t1032: F, t6235: F) -> (F, F, F, F, F, F, F, F) {
    let t19607 = t3316 * t1678;
    let t19608 = t342 * t19607;
    let t19611 = t6299 * t73;
    let t19649 = t1065 * t6244;
    let t19658 = t3172 * t6301;
    let t19659 = t1041 * t19658;
    let t19675 = t1065 * t6258;
    let t19696 = t6235 * t1032;
    (t19607, t19608, t19611, t19649, t19658, t19659, t19675, t19696)
}
