//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 474/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk474<F: Float>(t1614: F, t2606: F, t1451: F, t1611: F, t1627: F, t1639: F, t1644: F, t1649: F, t1651: F, t2559: F, t2568: F, t2571: F, t2580: F, t2583: F, t2587: F, t2596: F, t307: F, t311: F, t319: F, t328: F) -> (F, F) {
    let t2607 = t2606 * t1614;
    let t2610 = t2559 * t1611 / F::cast_from(12.0_f64) - t2568 * t311 / F::cast_from(6.0_f64) - t2571 * t311 / F::cast_from(6.0_f64) - t2580 * t311 / F::cast_from(6.0_f64) - t2583 * t311 / F::cast_from(6.0_f64) + t319 * t2587 / F::cast_from(6.0_f64) - t2596 * t1451 / F::cast_from(6.0_f64) - t328 * t2587 / F::cast_from(6.0_f64) + t307 * t2587 / F::cast_from(6.0_f64) - t2607 * t1451 / F::cast_from(6.0_f64) + t1627 - t1639 + t1644 - t1649 - t1651;
    (t2607, t2610)
}
