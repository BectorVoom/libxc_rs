//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 82/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk82<F: Float>(t160: F, t162: F, t148: F, t149: F, t151: F, t159: F) -> (F, F) {
    let t163 = t160 * t162;
    let t166 = F::new(1.0) + F::new(0.86931614897887578546e-1) * t148 * t149 * t151 + F::new(0.75571056687546295931e-2) * t159 * t163;
    let t167 = F::new(1.0) / t166;
    (t166, t167)
}
