//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1770/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1770<F: Float>(t422: F, t90614: F, t90626: F, t20400: F, t6556: F, t1196: F, t24408: F, t5197: F, t24473: F, t5192: F, t1188: F, t12485: F, t90357: F) -> (F, F, F, F, F) {
    let t90629 = F::cast_from(0.621814e-1_f64) * (t90614 + t90626) * t422;
    let t90631 = F::cast_from(0.10389515463408878255e3_f64) * t20400 * t6556;
    let t90634 = F::cast_from(0.46785788981077169656e1_f64) * t1196 * t5197 * t24408;
    let t90636 = F::cast_from(0.20779030926817756511e3_f64) * t5192 * t24473;
    let t90640 = F::cast_from(0.14035736694323150897e2_f64) * t1196 * t12485 * t90357 * t1188;
    (t90629, t90631, t90634, t90636, t90640)
}
