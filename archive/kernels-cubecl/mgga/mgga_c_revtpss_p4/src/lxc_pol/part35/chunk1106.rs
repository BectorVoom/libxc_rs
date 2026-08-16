//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1106/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1106<F: Float>(t11006: F, t256: F, t10115: F, t251: F, t2410: F, t90: F, t29: F, t560: F, t9655: F, t1389: F, t268: F, t555: F) -> (F, F, F, F, F, F, F) {
    let t41077 = F::cast_from(1.0_f64) / t11006 / t256;
    let t41117 = t10115 * t251;
    let t41153 = t2410 * t2410;
    let t41154 = F::cast_from(1.0_f64) / t41153;
    let t45970 = t90 * t90;
    let t45972 = t29 / t45970;
    let t46361 = F::cast_from(1.0_f64) / t9655 / t560;
    let t46808 = t1389 * t268;
    let t47567 = t10115 * t555;
    (t41077, t41117, t41154, t45972, t46361, t46808, t47567)
}
