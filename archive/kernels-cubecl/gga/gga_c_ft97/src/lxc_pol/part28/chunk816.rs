//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 816/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk816<F: Float>(t23470: F, t5943: F, t604: F, t7407: F, t379: F, t2210: F, t160: F, t7339: F, t2221: F, t609: F, t2179: F, t144: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t33031 = t23470 * t5943;
    let t33034 = t604 * t7407;
    let t33035 = t33034 * t379;
    let t33036 = t2210 * t33035;
    let t33039 = t160 * t7339;
    let t33040 = t33039 * t379;
    let t33041 = t2221 * t33040;
    let t33044 = t7407 * t609;
    let t33045 = t2179 * t33044;
    let t33046 = t144 * t33045;
    (t33031, t33034, t33035, t33036, t33039, t33040, t33041, t33044, t33045, t33046)
}
