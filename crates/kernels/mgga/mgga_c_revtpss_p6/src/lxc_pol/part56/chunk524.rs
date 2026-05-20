//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 524/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk524<F: Float>(t1534: F, t177: F, t762: F, t162: F, t2611: F, t1469: F, t189: F, t606: F, t2623: F, t2621: F, t2628: F, t2632: F, t4307: F, t4310: F, t4313: F, t4316: F, t4394: F, t4396: F, t4397: F) -> (F, F, F, F) {
    let t4398 = t1534 * t177;
    let t4399 = t4398 * t762;
    let t4400 = F::cast_from(0.5848223622634646207e0_f64) * t4399;
    let t4401 = t2611 * t162;
    let t4402 = t189 * t1469;
    let t4403 = t4402 * t606;
    let t4405 = F::new(12.0) * t4401 * t4403;
    let t4406 = F::cast_from(0.18311447306006545054e-3_f64) * t2623;
    let t4407 = t4307 + t4310 + t4313 + t4316 + t2632 + t2628 + t4394 + t4396 + t4397 - t4400 + t4405 + t2621 - t4406;
    (t4400, t4405, t4406, t4407)
}
