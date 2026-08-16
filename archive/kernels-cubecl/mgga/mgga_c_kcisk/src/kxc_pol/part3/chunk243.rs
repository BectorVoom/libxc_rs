//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 243/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk243<F: Float>(t264: F, t281: F, t259: F, t67: F, t852: F, t10: F, t142: F, t260: F, t261: F, t116: F) -> (F, F, F, F, F) {
    let t265 = t264 < -F::cast_from(0.66725e-1_f64);
    let t1099 = t281 * t281;
    let t1100 = F::cast_from(1.0_f64) / t1099;
    let t1101 = t259 * t1100;
    let t1102 = t67 * t852;
    let t1110 = piecewise3::<F>(t265, F::cast_from(0.0_f64), F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t260 * t1102 * t10 - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t260 * t261 * t142);
    let t1111 = t1110 * t116;
    (t1099, t1100, t1101, t1102, t1111)
}
