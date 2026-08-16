//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 948/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk948<F: Float>(t1101: F, t2160: F, t2158: F, t2799: F, t898: F, t2801: F, t3947: F, t3952: F, t11062: F, t283: F, t8647: F, t8651: F, t8655: F, t8659: F, t8663: F, t8668: F, t8670: F, t8673: F, t8675: F, t8684: F) -> F {
    let t11090 = t1101 * t2160;
    let t11092 = t1101 * t2158;
    let t11093 = F::cast_from(60.0_f64) * t11092;
    let t11095 = t2799 * t898;
    let t11097 = t2801 * t898;
    let t11098 = F::cast_from(144.0_f64) * t11097;
    let t11099 = t3947 * t898;
    let t11100 = F::cast_from(240.0_f64) * t11099;
    let t11101 = t3952 * t898;
    let t11104 = -t8647 - t8651 + t8655 + t8659 + F::cast_from(0.0007324578922402618_f64) * t8663 + t8668 - F::cast_from(0.00018311447306006544_f64) * t8670 + F::cast_from(0.0197516734986138_f64) * t11062 * t283 + F::cast_from(60.0_f64) * t11090 + t11093 - F::cast_from(0.5848223622634646_f64) * t8673 + F::cast_from(24.0_f64) * t11095 - t11098 + t11100 - F::cast_from(120.0_f64) * t11101 + F::cast_from(3.5089341735807875_f64) * t8675 - t8684;
    t11104
}
