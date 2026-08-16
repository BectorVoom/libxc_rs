//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 499/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk499<F: Float>(t1936: F, t205: F, t2414: F, t208: F, t1998: F, t1679: F, t1682: F, t1700: F, t1703: F, t1939: F, t213: F, t224: F, t2519: F, t2522: F, t2523: F, t2524: F) -> (F, F, F, F, F) {
    let t2525 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t1936;
    let t2526 = t2414 * t205;
    let t2527 = t2526 * t208;
    let t2531 = F::cast_from(4.0_f64) / F::cast_from(135.0_f64) * t1998;
    let t2532 = t1679 - t1682 + t1700 + t1703 - t2519 * t224 / F::cast_from(15.0_f64) + t2522 + t2523 + t2524 + t2525 + t2527 * t213 / F::cast_from(3.0_f64) - F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t1939 + t2531;
    (t2525, t2526, t2527, t2531, t2532)
}
