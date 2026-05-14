//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 899/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk899<F: Float>(t3424: F, t5916: F, t23667: F, t5899: F, t23892: F, t3188: F, t23671: F, t1642: F, t586: F) -> (F, F, F, F, F, F, F) {
    let t27064 = t5916 * t3424;
    let t27065 = t23667 * t27064;
    let t27066 = t5899 * t27065;
    let t27068 = t23892 * t3188;
    let t27069 = t23671 * t27068;
    let t27070 = t5899 * t27069;
    let t27072 = t1642 * t586;
    (t27064, t27065, t27066, t27068, t27069, t27070, t27072)
}
