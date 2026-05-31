//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1101/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1101<F: Float>(t3269: F, t39203: F, t3579: F, t37286: F, t11531: F, t792: F, t10997: F, t3275: F, t11584: F, t37365: F, t10673: F, t11587: F, t37360: F) -> (F, F, F, F, F) {
    let t39205 = t3269 * t39203 / F::cast_from(2.0_f64);
    let t39208 = t3579 * t37286 / F::cast_from(2.0_f64);
    let t39209 = t11531 * t792;
    let t39212 = F::cast_from(45.0_f64) / F::cast_from(32.0_f64) * t3275 * t10997 * t39209;
    let t39215 = t37365 * t11584;
    let t39218 = t10673 * t11587 * t37360;
    (t39205, t39208, t39212, t39215, t39218)
}
