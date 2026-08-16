//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1114/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1114<F: Float>(t27055: F, t7772: F, t26779: F, t26787: F, t26798: F, t26801: F, t26804: F, t26812: F, t26826: F, t26829: F, t26834: F, t26838: F, t27020: F, t27042: F, t27053: F, t7775: F) -> (F, F) {
    let t27056 = t7772 * t27055;
    let t27058 = F::cast_from(0.92858888888888888886e-2_f64) * t26779 + F::cast_from(0.15476481481481481481e-2_f64) * t26787 - F::cast_from(0.34822083333333333332e-2_f64) * t26798 - F::cast_from(0.24734586805555555556e-3_f64) * t27042 * t7775 - F::cast_from(0.23214722222222222222e-2_f64) * t26801 + F::cast_from(0.15476481481481481481e-2_f64) * t26804 + F::cast_from(0.46377350260416666667e-4_f64) * t7772 * t27020 - F::cast_from(0.17411041666666666666e-2_f64) * t26812 + F::cast_from(0.23214722222222222222e-2_f64) * t26826 + F::cast_from(0.17411041666666666666e-2_f64) * t26829 + F::cast_from(0.17024129629629629629e-1_f64) * t26834 - t27053 - F::cast_from(0.61905925925925925925e-2_f64) * t26838 + F::cast_from(0.30918233506944444444e-4_f64) * t27056;
    (t27056, t27058)
}
