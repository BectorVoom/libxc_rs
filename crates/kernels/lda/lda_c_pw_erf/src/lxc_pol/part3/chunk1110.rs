//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1110/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1110<F: Float>(t2070: F, t493: F, t785: F, t11898: F, t2130: F, t1318: F, t3899: F, t4942: F, t1466: F, t3667: F, t3669: F, t811: F) -> (F, F, F, F) {
    let t12984 = t493 * t2070 * t785;
    let t12985 = F::new(32.0) / F::new(405.0) * t12984;
    let t12987 = t493 * t11898 * t2130;
    let t12988 = F::new(64.0) / F::new(45.0) * t12987;
    let t12990 = t1318 * t3899 * t4942;
    let t12991 = F::new(16.0) / F::new(15.0) * t12990;
    let t12996 = F::new(8.0) / F::new(5.0) * t1318 * t1466 * t3667 * t811 * t3669;
    (t12985, t12988, t12991, t12996)
}
