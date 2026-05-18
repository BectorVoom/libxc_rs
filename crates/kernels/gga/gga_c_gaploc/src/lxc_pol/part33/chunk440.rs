//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 440/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk440<F: Float>(t1477: F, t2124: F, t14: F, t257: F, t752: F, t667: F, t78: F, t4: F, t1487: F, t754: F, t110: F, t656: F) -> (F, F, F, F, F) {
    let t2125 = t2124 * t1477;
    let t2128 = t14 * t257;
    let t2129 = t752 * t2128;
    let t2130 = t78 * t667;
    let t2131 = t4 * t2130;
    let t2134 = t754 * t1487;
    let t2137 = t110 * t656;
    (t2125, t2129, t2131, t2134, t2137)
}
