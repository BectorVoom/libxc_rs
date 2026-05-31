//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1101/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1101<F: Float>(t1887: F, t2897: F, t1792: F, t2888: F, t12114: F, t12133: F, t1888: F, t534: F, t6849: F, t6853: F, t1896: F, t452: F) -> F {
    let t12135 = t2897 * t1887;
    let t12140 = t2888 * t1792;
    let t12145 = t12133 * t534 - t12135 * t1792 / F::cast_from(2.0_f64) - t6849 * t2888 / F::cast_from(2.0_f64) + F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t6853 * t12140 - t1888 * t12114 / F::cast_from(2.0_f64);
    let t12146 = t12145 * t1896;
    let t12147 = t12146 * t452;
    t12147
}
