//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 964/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk964<F: Float>(t1580: F, t2487: F, t10025: F, t10182: F, t10184: F, t10187: F, t10190: F, t10193: F, t10199: F, t10201: F, t10204: F, t10206: F, t10209: F, t10216: F, t1629: F, t2587: F, t311: F, t5422: F, t5786: F, t5800: F, t5803: F, t5806: F, t5812: F, t5815: F, t5817: F) -> F {
    let t10219 = t1580 * t2487;
    let t10222 = F::cast_from(0.14975624337724558_f64) * t10182 + F::cast_from(0.02466859483068398_f64) * t10184 - F::cast_from(0.02466859483068398_f64) * t10187 - F::cast_from(0.14975624337724558_f64) * t5422 + t10190 * t1629 / F::cast_from(6.0_f64) - t10193 * t10025 / F::cast_from(3.0_f64) + t5786 * t2587 / F::cast_from(6.0_f64) + t10199 / F::cast_from(6.0_f64) + t10201 * t1629 / F::cast_from(6.0_f64) + t10204 / F::cast_from(6.0_f64) + t10206 * t10025 / F::cast_from(3.0_f64) - t10209 / F::cast_from(6.0_f64) + t5800 / F::cast_from(6.0_f64) - t5803 / F::cast_from(6.0_f64) - t5806 / F::cast_from(6.0_f64) - t5812 - t5815 / F::cast_from(12.0_f64) + t5817 / F::cast_from(18.0_f64) - t10216 * t311 / F::cast_from(6.0_f64) - t10219 * t311 / F::cast_from(6.0_f64);
    t10222
}
