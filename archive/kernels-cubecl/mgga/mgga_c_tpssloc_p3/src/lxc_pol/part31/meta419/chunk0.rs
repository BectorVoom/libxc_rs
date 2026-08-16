//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1530/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1530<F: Float>(t20118: F, t20147: F, t3: F, t112: F, t6470: F, t576: F, t671: F, t1458: F, t4072: F, t5493: F, t12524: F, t1401: F, t16521: F, t16524: F, t19534: F, t3938: F, t3941: F, t5371: F, t5376: F, t5456: F, t577: F) -> (F, F, F, F, F, F, F) {
    let t20148 = t20118 + t20147;
    let t20149 = t3 * t20148;
    let t20162 = t6470 * t112;
    let t20173 = t576 * t671;
    let t20176 = t1458 * t4072;
    let t20181 = t5493 * t671;
    let t20186 = F::cast_from(0.45e1_f64) * t20148 * t577 + F::cast_from(0.135e2_f64) * t20162 * t671 + F::cast_from(27.0_f64) * t16521 * t1458 + F::cast_from(54.0_f64) * t16524 * t5376 + F::cast_from(27.0_f64) * t5371 * t4072 + F::cast_from(27.0_f64) * t12524 * t5456 + F::cast_from(27.0_f64) * t20173 * t5456 + F::cast_from(54.0_f64) * t3941 * t20176 + F::cast_from(0.135e2_f64) * t3938 * t5493 + F::cast_from(27.0_f64) * t3941 * t20181 + F::cast_from(0.135e2_f64) * t1401 * t19534;
    (t20148, t20149, t20162, t20173, t20176, t20181, t20186)
}
