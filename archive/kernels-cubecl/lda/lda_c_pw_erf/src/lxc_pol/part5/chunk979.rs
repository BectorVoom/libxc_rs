//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 979/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk979<F: Float>(t1081: F, t5701: F, t1772: F, t3007: F, t5677: F, t684: F, t5681: F, t1738: F, t2306: F, t1729: F, t1880: F, t405: F, t6153: F) -> (F, F, F, F, F, F, F) {
    let t14447 = t5701 * t1081;
    let t14448 = F::cast_from(0.0007324622014701264_f64) * t14447;
    let t14449 = t1772 * t3007;
    let t14468 = t684 * t5677;
    let t14469 = F::cast_from(0.11974234010254609_f64) * t14468;
    let t14470 = t684 * t5681;
    let t14472 = t1738 * t2306;
    let t14473 = F::cast_from(0.15965645347006147_f64) * t14472;
    let t14480 = t1729 * t1880;
    let t14485 = t405 * t6153;
    (t14448, t14449, t14469, t14470, t14473, t14480, t14485)
}
