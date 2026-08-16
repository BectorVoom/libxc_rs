//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1292/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1292<F: Float>(t20469: F, t422: F, t12485: F, t6518: F, t5206: F, t1196: F, t5192: F, t5198: F, t20283: F, t20285: F, t20287: F, t20290: F, t20295: F, t20300: F, t20304: F, t20308: F, t20312: F, t20315: F, t20320: F) -> (F, F, F, F) {
    let t20471 = F::cast_from(0.621814e-1_f64) * t20469 * t422;
    let t20472 = t12485 * t6518;
    let t20473 = t20472 * t5206;
    let t20475 = F::cast_from(0.10389515463408878255e3_f64) * t1196 * t20473;
    let t20477 = F::cast_from(0.23392894490538584828e1_f64) * t5192 * t5198;
    let t20498 = F::cast_from(0.11477222222222222222e0_f64) * t20283 - F::cast_from(0.34431666666666666667e0_f64) * t20285 - F::cast_from(0.17215833333333333333e0_f64) * t20287 + F::cast_from(0.516475e0_f64) * t20290 + F::cast_from(0.57386111111111111112e0_f64) * t20295 - F::cast_from(0.20659e1_f64) * t20300 - F::cast_from(0.68863333333333333334e0_f64) * t20304 + F::cast_from(0.309885e1_f64) * t20308 + F::cast_from(0.20659e1_f64) * t20312 - F::cast_from(0.34431666666666666667e0_f64) * t20315 + F::cast_from(0.103295e1_f64) * t20320;
    (t20471, t20475, t20477, t20498)
}
