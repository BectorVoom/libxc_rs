//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 816/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk816<F: Float>(t1190: F, t12727: F, t1159: F, t3035: F, t1162: F, t1165: F, t3211: F, t407: F, t3375: F, t3445: F, t3073: F, t3371: F, t3459: F, t160: F, t972: F, t1170: F, t1171: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12728 = t12727 * t1190;
    let t12730 = t3035 * t1159;
    let t12731 = t12730 * t1162;
    let t12734 = t12731 * t1165 * t3211 * t407;
    let t12736 = t3375 * t3445;
    let t12738 = t3073 * t3371;
    let t12739 = t12738 * t3459;
    let t12741 = t160 * t972;
    let t12743 = t1170 * t12741 * t1171;
    (t12728, t12730, t12731, t12734, t12736, t12738, t12739, t12741, t12743)
}
