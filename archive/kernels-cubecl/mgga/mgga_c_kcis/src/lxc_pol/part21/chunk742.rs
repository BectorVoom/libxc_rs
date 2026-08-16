//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 742/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk742<F: Float>(t2197: F, t7772: F, t7786: F, t7788: F, t7801: F, t8049: F, t8052: F, t8055: F, t8058: F, t8083: F, t8087: F, t8091: F, t8095: F) -> F {
    let t8104 = -F::cast_from(0.34752604166666666667e-3_f64) * t8083 * t2197 + F::cast_from(0.46377350260416666667e-4_f64) * t7772 * t8087 - t7786 - F::cast_from(0.11584201388888888889e-3_f64) * t7788 * t8091 + F::cast_from(0.34752604166666666667e-3_f64) * t7788 * t8095 + F::cast_from(0.34752604166666666667e-3_f64) * t7788 * t8087 + t7801 + F::cast_from(0.11607361111111111111e-2_f64) * t8049 + F::cast_from(0.17411041666666666666e-2_f64) * t8052 - F::cast_from(0.17411041666666666666e-2_f64) * t8055 + F::cast_from(0.11607361111111111111e-2_f64) * t8058;
    t8104
}
