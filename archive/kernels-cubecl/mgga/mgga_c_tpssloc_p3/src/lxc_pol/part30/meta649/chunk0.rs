//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2063/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2063<F: Float>(t23384: F, t25518: F, t10277: F, t381: F, t225: F, t25608: F, t25714: F, t7604: F, t82573: F, t25718: F, t23665: F, t25541: F) -> (F, F, F, F, F, F, F) {
    let t89057 = F::cast_from(0.18277045187202515961e-2_f64) * t23384 * t25518;
    let t89071 = t381 * t10277;
    let t89076 = t25608 * t225;
    let t89094 = F::cast_from(0.54831135561607547884e-2_f64) * t23384 * t25714;
    let t89104 = t82573 * t7604;
    let t89151 = F::cast_from(0.18277045187202515961e-2_f64) * t23384 * t25718;
    let t89156 = F::cast_from(0.54831135561607547884e-2_f64) * t23665 * t25541;
    (t89057, t89071, t89076, t89094, t89104, t89151, t89156)
}
