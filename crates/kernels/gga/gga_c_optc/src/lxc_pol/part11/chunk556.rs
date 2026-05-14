//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 556/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk556<F: Float>(t127: F, t4649: F, t6: F, t161: F, t4624: F, t141: F, t4599: F, t2087: F) -> (F, F, F, F, F) {
    let t4651 = t6 * t4649 * t127;
    let t4652 = t161 * t4651;
    let t4655 = t4624 * t127;
    let t4656 = t161 * t4655;
    let t4660 = t141 * t4599;
    let t4661 = t2087 * t4660;
    (t4651, t4652, t4655, t4656, t4661)
}
