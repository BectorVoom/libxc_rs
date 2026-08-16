//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1167/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1167<F: Float>(t28093: F, t7788: F, t26826: F, t26838: F, t26846: F, t26966: F, t27014: F, t27053: F, t27056: F, t27080: F, t27087: F, t27926: F, t27929: F, t27932: F, t27941: F, t27947: F, t27969: F, t27972: F, t28132: F, t28137: F, t28153: F, t7772: F, t8087: F) -> F {
    let t28235 = t7788 * t28093;
    let t28248 = F::cast_from(0.46377350260416666667e-4_f64) * t7772 * t28153 - F::cast_from(0.17411041666666666666e-2_f64) * t27926 + F::cast_from(0.11607361111111111111e-2_f64) * t27929 - F::cast_from(0.17411041666666666666e-2_f64) * t27932 + F::cast_from(0.11607361111111111111e-2_f64) * t26826 - F::cast_from(0.46429444444444444443e-2_f64) * t27941 - t27053 - F::cast_from(0.30952962962962962963e-2_f64) * t26838 + F::cast_from(0.15459116753472222222e-4_f64) * t27056 - F::cast_from(0.11607361111111111111e-2_f64) * t26846 - F::cast_from(0.92673611111111111112e-3_f64) * t26966 * t8087 + F::cast_from(0.11584201388888888889e-3_f64) * t28235 - F::cast_from(0.34752604166666666667e-3_f64) * t7788 * t28132 - F::cast_from(0.69505208333333333334e-3_f64) * t7788 * t28137 + F::cast_from(0.11607361111111111111e-2_f64) * t27947 + F::cast_from(0.30891203703703703704e-3_f64) * t27080 + F::cast_from(0.34752604166666666667e-3_f64) * t27014 * t8087 + F::cast_from(0.11584201388888888889e-3_f64) * t27087 - F::cast_from(0.11607361111111111111e-2_f64) * t27969 + F::cast_from(0.46429444444444444443e-2_f64) * t27972;
    t28248
}
