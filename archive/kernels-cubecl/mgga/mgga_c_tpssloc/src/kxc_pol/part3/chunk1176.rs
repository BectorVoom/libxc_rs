//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 1176/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk1176<F: Float>(t15420: F, t3447: F, t11514: F, t11556: F, t11558: F, t11561: F, t15391: F, t15396: F, t15401: F, t15405: F, t15406: F, t15409: F, t15412: F, t15415: F) -> F {
    let t15422 = F::cast_from(0.24691358024691358024e-3_f64) * t3447 * t15420;
    let t15423 = -F::cast_from(0.27777777777777777777e-3_f64) * t11514 + F::cast_from(0.37037037037037037036e-3_f64) * t11558 - F::cast_from(0.27777777777777777777e-3_f64) * t11561 + t11556 - F::cast_from(0.37037037037037037036e-3_f64) * t3447 * t15391 - F::cast_from(0.86419753086419753084e-3_f64) * t3447 * t15396 + t15401 - t15405 + F::cast_from(0.74074074074074074072e-3_f64) * t3447 * t15406 + F::cast_from(0.37037037037037037036e-3_f64) * t3447 * t15409 + F::cast_from(0.22222222222222222221e-2_f64) * t3447 * t15412 + F::cast_from(0.27777777777777777777e-3_f64) * t3447 * t15415 + t15422;
    t15423
}
