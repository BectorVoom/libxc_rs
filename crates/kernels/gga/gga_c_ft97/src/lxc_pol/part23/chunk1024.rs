//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1024/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1024<F: Float>(t3977: F, t6921: F, t729: F, t2574: F, t265: F, t31014: F, t1449: F, t4934: F, t762: F, t1175: F, t6852: F, t10157: F, t31036: F, t5053: F, t1131: F, t6940: F) -> (F, F, F, F, F, F, F, F, F) {
    let t31213 = t729 * t3977 * t6921;
    let t31217 = t2574 * t265 * t31014;
    let t31220 = t1449 * t4934;
    let t31222 = t2574 * t762 * t31220;
    let t31226 = t2574 * t1175 * t6852;
    let t31231 = t10157 * t265 * t31036;
    let t31234 = t1449 * t5053;
    let t31236 = t729 * t762 * t31234;
    let t31239 = t6940 * t1131;
    (t31213, t31217, t31220, t31222, t31226, t31231, t31234, t31236, t31239)
}
