//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 891/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk891<F: Float>(t3395: F, t3403: F, t1155: F, t1138: F, t3351: F, t1136: F, t3359: F, t11135: F, t11137: F, t11139: F, t11141: F, t11143: F, t11150: F, t11156: F, t11161: F, t11165: F, t11170: F, t11174: F) -> (F, F, F, F) {
    let t11433 = t3395 * t3403;
    let t11434 = t11433 * t1155;
    let t11437 = t1138 * t3351;
    let t11441 = t3351 * t3359 * t1136;
    let t11444 = F::cast_from(0.53272592592592592592e-1_f64) * t11135;
    let t11455 = -t11444 + F::cast_from(0.2283111111111111111e-1_f64) * t11137 + F::cast_from(0.11415555555555555555e-1_f64) * t11139 - F::cast_from(0.34246666666666666665e-1_f64) * t11141 - F::cast_from(0.17123333333333333333e-1_f64) * t11143 + F::cast_from(0.19025925925925925925e-1_f64) * t11150 - F::cast_from(0.68493333333333333331e-1_f64) * t11156 - F::cast_from(0.34246666666666666665e-1_f64) * t11161 + F::cast_from(0.10274e0_f64) * t11165 + F::cast_from(0.10274e0_f64) * t11170 + F::cast_from(0.17123333333333333333e-1_f64) * t11174;
    (t11434, t11437, t11441, t11455)
}
