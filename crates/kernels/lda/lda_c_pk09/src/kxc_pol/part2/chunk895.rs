//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 895/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk895<F: Float>(t1067: F, t2337: F, t1047: F, t4459: F, t4461: F, t4475: F, t8617: F, t8621: F, t8859: F, t8863: F, t8867: F, t8871: F, t9443: F, t9446: F, t9449: F, t9453: F, t9459: F, t98: F) -> F {
    let t9461 = t2337 * t1067;
    let t9467 = -t9443 / F::new(9.0) - t9446 * t98 / F::new(6.0) - t9449 * t98 / F::new(6.0) - t4459 + t4461 + t9453 * t8617 / F::new(3.0) + t8621 * t1047 / F::new(36.0) - F::cast_from(0.14975624337724558_f64) * t4475 + t9459 / F::new(9.0) - t9461 / F::new(9.0) - F::cast_from(0.01233429741534199_f64) * t8859 + F::cast_from(0.01233429741534199_f64) * t8863 + F::cast_from(0.01233429741534199_f64) * t8867 - F::cast_from(0.14975624337724558_f64) * t8871;
    t9467
}
