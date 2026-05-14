//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1264/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1264<F: Float>(t23671: F, t27157: F, t30186: F, t379: F, t105435: F, t27147: F, t6656: F, t17006: F, t5916: F, t23667: F, t5899: F, t16011: F, t23892: F, t23909: F, t27072: F, t23657: F, t4431: F, t590: F) -> (F, F, F, F, F, F, F, F, F) {
    let t119602 = t27157 * t23671 * t30186 * t379;
    let t119606 = t27157 * t105435 * t6656 * t27147;
    let t119608 = t5916 * t17006;
    let t119610 = t5899 * t23667 * t119608;
    let t119612 = t23892 * t16011;
    let t119614 = t5899 * t23671 * t119612;
    let t119616 = t23909 * t16011;
    let t119618 = t5899 * t27072 * t119616;
    let t119623 = t23657 * t23671 * t5916 * t4431 * t590;
    (t119602, t119606, t119608, t119610, t119612, t119614, t119616, t119618, t119623)
}
