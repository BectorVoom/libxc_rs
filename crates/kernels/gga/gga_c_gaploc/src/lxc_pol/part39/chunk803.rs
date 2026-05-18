//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 803/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk803<F: Float>(t13945: F, t270: F, t13177: F, t13184: F, t13196: F, t13202: F, t13208: F, t13211: F, t13214: F, t13219: F, t13223: F, t13226: F, t13935: F, t13938: F, t13944: F) -> F {
    let t13947 = F::new(0.76905262301422242837e-2) * t270 * t13945;
    let t13948 = t13208 + t13211 - t13214 - t13219 + t13223 + F::new(0.32043859292259267849e-3) * t13177 + t13184 + t13196 - t13202 - F::new(0.32043859292259267849e-3) * t13226 - F::new(0.32043859292259267849e-3) * t13935 + F::new(0.32043859292259267849e-3) * t13938 + t13944 - t13947;
    t13948
}
