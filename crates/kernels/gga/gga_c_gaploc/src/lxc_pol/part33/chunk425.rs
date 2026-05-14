//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 425/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk425<F: Float>(t2109: F, t322: F, t178: F, t1832: F, t108: F, t670: F, t14: F, t260: F, t435: F, t341: F, t19: F, t271: F, t1477: F, t257: F, t752: F, t667: F, t78: F) -> (F, F, F, F, F, F, F) {
    let t2110 = t2109 * t322;
    let t2113 = t1832 * t178;
    let t2116 = t670 * t108;
    let t2117 = t2116 * t14;
    let t2122 = t260 * t435;
    let t2123 = t2122 * t341;
    let t2124 = t271 * t19;
    let t2125 = t2124 * t1477;
    let t2128 = t14 * t257;
    let t2129 = t752 * t2128;
    let t2130 = t78 * t667;
    (t2110, t2113, t2117, t2123, t2125, t2129, t2130)
}
