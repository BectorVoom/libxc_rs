//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1017/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1017<F: Float>(t1284: F, t7838: F, t39: F, t8327: F, t186: F, t220: F, t548: F, t1982: F, t2499: F, t2505: F, t6580: F, t6568: F, t795: F, t2120: F, t6592: F, t16971: F) -> (F, F, F, F, F, F, F, F) {
    let t21296 = 4.0 / 15.0 * t1284 * t7838;
    let t21299 = -6.0 * t39 - 12.0 * t8327;
    let t21303 = 4.0 / 15.0 * t548 * t186 * t220 * t21299;
    let t21305 = 2.0 / 5.0 * t1982 * t2499;
    let t21307 = 4.0 / 5.0 * t6580 * t2505;
    let t21309 = 2.0 / 5.0 * t795 * t6568;
    let t21311 = 4.0 / 5.0 * t2120 * t6592;
    let t21313 = 4.0 / 5.0 * t16971 * t2505;
    (t21296, t21299, t21303, t21305, t21307, t21309, t21311, t21313)
}
