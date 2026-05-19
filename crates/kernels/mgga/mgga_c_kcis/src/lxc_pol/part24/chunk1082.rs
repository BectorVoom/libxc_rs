//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1082/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1082<F: Float>(t5336: F, t7794: F, t5329: F, t3500: F, t8090: F, t7788: F, t15198: F, t2192: F, t2197: F, t26751: F, t26787: F, t26985: F, t27007: F, t27014: F, t27024: F, t27070: F, t27880: F, t27883: F, t28176: F, t28179: F, t28184: F, t28190: F, t28204: F, t7772: F, t7775: F, t7791: F, t7796: F, t8087: F, t8095: F) -> (F, F, F, F, F, F) {
    let t28210 = t7794 * t5336;
    let t28211 = t5329 * t28210;
    let t28214 = t3500 * t8090;
    let t28215 = t7788 * t28214;
    let t28219 = t15198 * t2192;
    let t28222 = -F::cast_from(0.11584201388888888889e-3_f64) * t28176 - F::cast_from(0.69505208333333333334e-3_f64) * t7788 * t28179 - F::cast_from(0.34752604166666666667e-3_f64) * t7788 * t28184 - F::cast_from(0.46377350260416666667e-4_f64) * t7772 * t28184 - F::cast_from(0.11584201388888888889e-3_f64) * t28190 * t7791 + F::cast_from(0.46377350260416666667e-4_f64) * t27070 * t8087 - F::cast_from(0.11584201388888888889e-3_f64) * t26985 + F::cast_from(0.17411041666666666666e-2_f64) * t27880 - F::cast_from(0.17411041666666666666e-2_f64) * t27883 + F::cast_from(0.34752604166666666667e-3_f64) * t28190 * t7796 - F::cast_from(0.3861400462962962963e-4_f64) * t27007 + F::cast_from(0.34752604166666666667e-3_f64) * t28190 * t7775 + F::cast_from(0.46377350260416666667e-4_f64) * t28204 * t7775 + F::cast_from(0.11584201388888888889e-3_f64) * t27024 + F::cast_from(0.34752604166666666667e-3_f64) * t27014 * t8095 + F::cast_from(0.34752604166666666667e-3_f64) * t7788 * t28211 - F::cast_from(0.3861400462962962963e-4_f64) * t28215 + F::cast_from(0.77382407407407407407e-3_f64) * t26751 + F::cast_from(0.77382407407407407407e-3_f64) * t26787 - F::cast_from(0.34752604166666666667e-3_f64) * t28219 * t2197;
    (t28210, t28211, t28214, t28215, t28219, t28222)
}
