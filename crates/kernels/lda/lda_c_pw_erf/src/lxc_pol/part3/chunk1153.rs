//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1153/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1153<F: Float>(t2163: F, t3742: F, t1466: F, t3669: F, t571: F, t833: F, t9237: F, t1318: F, t2157: F, t9432: F, t3732: F, t4738: F) -> (F, F, F, F) {
    let t13500 = F::cast_from(8.0_f64) / F::cast_from(5.0_f64) * t3742 * t2163;
    let t13505 = F::cast_from(16.0_f64) / F::cast_from(5.0_f64) * t571 * t1466 * t9237 * t833 * t3669;
    let t13507 = t1318 * t9432 * t2157;
    let t13508 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t13507;
    let t13510 = F::cast_from(8.0_f64) / F::cast_from(5.0_f64) * t4738 * t3732;
    (t13500, t13505, t13508, t13510)
}
