//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 729/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk729<F: Float>(t13202: F, t13208: F, t13211: F, t13214: F, t13215: F, t13216: F, t13219: F, t13220: F, t13223: F, t13944: F, t13947: F, t14489: F) -> F {
    let t14490 = -t13202 + t13208 + t13211 - t13214 - t13215 + t13216 - t13219 + t13220 + t13223 + t13944 - t13947;
    let t14491 = t14489 + t14490;
    t14491
}
