//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 778/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk778<F: Float>(t12351: F, t12295: F, t3475: F, t431: F, t426: F, t1159: F, t3478: F, t434: F, t3519: F, t444: F, t439: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12352 = F::new(0.36514074074074074075e0) * t12351;
    let t12367 = F::new(0.28842592592592592592e-1) * t12295;
    let t12382 = F::new(0.55403703703703703703e-1) * t12295;
    let t12397 = F::new(0.53272592592592592592e-1) * t12295;
    let t12428 = F::new(1.0) / t3475 / t431;
    let t12429 = t426 * t12428;
    let t12459 = F::new(0.16068111111111111111e1) * t12295;
    let t12460 = F::new(0.46308888888888888888e0) * t12351;
    let t12469 = F::new(1.0) / t3475 / t1159;
    let t12470 = t426 * t12469;
    let t12472 = F::new(1.0) / t3478 / t434;
    let t12485 = F::new(1.0) / t3519 / t444;
    let t12486 = t439 * t12485;
    let t12542 = F::new(0.93932222222222222223e0) * t12295;
    (t12352, t12367, t12382, t12397, t12429, t12459, t12460, t12470, t12472, t12485, t12486, t12542)
}
