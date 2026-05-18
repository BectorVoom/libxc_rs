//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 709/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk709<F: Float>(t1871: F, t4436: F, t986: F, t16246: F, t979: F, t83: F, t110: F, t20098: F, t452: F, t4495: F, t20203: F, t8424: F) -> (F, F, F, F, F, F) {
    let t20417 = t1871 * t986 * t4436;
    let t20420 = t16246 * t979;
    let t20421 = t83 * t20420;
    let t20424 = t452 * t110 * t20098;
    let t20428 = t452 * t986 * t4495;
    let t20430 = t8424 * t20203;
    (t20417, t20420, t20421, t20424, t20428, t20430)
}
