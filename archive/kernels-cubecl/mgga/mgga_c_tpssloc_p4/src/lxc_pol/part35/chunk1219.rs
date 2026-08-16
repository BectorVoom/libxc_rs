//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1219/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1219<F: Float>(t5371: F, t7467: F, t5456: F, t576: F, t1873: F, t1458: F, t3941: F, t5493: F, t1401: F, t28017: F, t1409: F, t22510: F, t24498: F, t27356: F, t5392: F, t5398: F, t5415: F, t56: F, t7251: F) -> (F, F, F, F, F, F, F, F, F) {
    let t28892 = F::cast_from(27.0_f64) * t5371 * t7467;
    let t28893 = t576 * t5456;
    let t28895 = F::cast_from(27.0_f64) * t28893 * t1873;
    let t28896 = t7467 * t1458;
    let t28898 = F::cast_from(54.0_f64) * t3941 * t28896;
    let t28899 = t1873 * t5493;
    let t28901 = F::cast_from(27.0_f64) * t3941 * t28899;
    let t28903 = F::cast_from(0.135e2_f64) * t1401 * t28017;
    let t29473 = F::cast_from(88.0_f64) / F::cast_from(9.0_f64) * t5415 * t56 + F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t27356 * t1409 + F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t24498 * t5392 - F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t7251 * t5398 - t22510;
    (t28892, t28893, t28895, t28896, t28898, t28899, t28901, t28903, t29473)
}
