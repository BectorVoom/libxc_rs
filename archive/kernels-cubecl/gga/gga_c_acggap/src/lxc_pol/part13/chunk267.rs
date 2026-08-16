//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 267/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk267<F: Float>(t1008: F, t425: F, t431: F, t438: F, t173: F) -> (F, F, F, F, F) {
    let t1009 = t1008 * t425;
    let t1011 = t1008 * t431;
    let t1013 = t1008 * t438;
    let t1015 = t173 * t173;
    let t1016 = F::cast_from(1.0_f64) / t1015;
    (t1009, t1011, t1013, t1015, t1016)
}
