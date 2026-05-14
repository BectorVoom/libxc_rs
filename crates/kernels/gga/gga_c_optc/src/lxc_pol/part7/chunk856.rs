//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 856/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk856<F: Float>(t8642: F, t1040: F, t2941: F, t2950: F, t2958: F, t2869: F, t8498: F, t25: F) -> (F, F, F, F, F, F, F) {
    let t8643 = 0.36514074074074074075e0 * t8642;
    let t8644 = t2941 * t1040;
    let t8645 = t8644 * t2950;
    let t8647 = t2958 * t1040;
    let t8648 = t8647 * t2950;
    let t8650 = t2869 * t8498;
    let t8651 = t25 * t8650;
    (t8643, t8644, t8645, t8647, t8648, t8650, t8651)
}
