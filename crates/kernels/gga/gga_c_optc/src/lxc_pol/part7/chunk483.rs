//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 483/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk483<F: Float>(t2382: F, t2383: F, t2257: F, t2259: F, t2266: F, t2272: F, t2276: F) -> (F, F) {
    let t2384 = t2382 * t2383;
    let t2386 = F::new(4.0) / F::new(9.0) * t2257;
    let t2391 = t2386 + F::new(2.0) / F::new(9.0) * t2259 - F::new(2.0) / F::new(9.0) * t2266 + F::new(2.0) / F::new(3.0) * t2272 - t2276 / F::new(3.0);
    (t2384, t2391)
}
