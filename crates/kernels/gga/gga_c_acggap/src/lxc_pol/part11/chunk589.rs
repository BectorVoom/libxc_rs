//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 589/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk589<F: Float>(t1101: F, t1165: F, t530: F, t4282: F, t1470: F, t3409: F, t1410: F, t174: F, t1175: F, t1181: F, t1182: F, t435: F) -> (F, F, F, F, F, F, F) {
    let t4284 = t1165 * t530 * t1101;
    let t4285 = t4282 * t4284;
    let t4288 = F::cast_from(0.40015750243531754508e-2_f64) * t3409 * t1470;
    let t4289 = t174 * t1410;
    let t4291 = t1165 * t4289 * t1175;
    let t4295 = t1181 * t4289 * t1182;
    let t4298 = t435 * t1410;
    (t4284, t4285, t4288, t4289, t4291, t4295, t4298)
}
