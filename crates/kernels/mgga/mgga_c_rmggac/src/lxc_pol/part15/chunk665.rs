//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 665/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk665<F: Float>(t271: F, t71: F, t4789: F, t1985: F, t793: F, t1003: F, t1171: F, t226: F, t325: F, t3807: F, t120: F, t860: F, t108: F, t124: F) -> (F, F, F, F, F, F, F, F) {
    let t20925 = t271 * t71;
    let t20963 = t4789 * t71;
    let t22971 = t1985 * t793;
    let t24889 = t1003 * t1003;
    let t24890 = 1.0 / t24889;
    let t24983 = t1171 * t1171;
    let t24985 = 1.0 / t226 / t24983;
    let t25441 = t3807 * t325;
    let t25518 = t120 * t860;
    let t25525 = t108 * t124;
    (t20925, t20963, t22971, t24890, t24985, t25441, t25518, t25525)
}
