//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 922/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk922<F: Float>(t1131: F, t485: F, t7220: F, t2363: F, t717: F, t1138: F, t1597: F, t164: F, t6138: F, t684: F, t7071: F, t1553: F, t1878: F, t2610: F, t2765: F, t440: F) -> (F, F, F, F, F, F, F) {
    let t18782 = t7220 * t1131 * t485;
    let t18784 = t717 * t2363;
    let t18786 = t18784 * t1138 * t1597;
    let t18788 = t6138 * t164;
    let t18795 = t684 * t7071;
    let t18797 = t1553 * t1878;
    let t18805 = t2765 * t2610 * t440;
    (t18782, t18784, t18786, t18788, t18795, t18797, t18805)
}
