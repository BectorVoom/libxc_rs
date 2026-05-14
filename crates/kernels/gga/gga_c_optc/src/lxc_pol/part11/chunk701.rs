//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 701/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk701<F: Float>(t8642: F, t8639: F, t1035: F, t3016: F, t375: F, t3019: F, t388: F, t1084: F, t3057: F) -> (F, F, F, F, F, F) {
    let t8643 = 0.36514074074074074075e0 * t8642;
    let t8662 = 28.0 / 27.0 * t8639;
    let t8685 = 1.0 / t3016 / t1035;
    let t8686 = t375 * t8685;
    let t8688 = 1.0 / t3019 / t388;
    let t8697 = 1.0 / t3057 / t1084;
    (t8643, t8662, t8685, t8686, t8688, t8697)
}
