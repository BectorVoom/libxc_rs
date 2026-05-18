//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 423/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk423<F: Float>(t228: F, t777: F, t216: F, t214: F, t217: F, t2257: F, t2280: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2371 = t777 * t228;
    let t2372 = F::new(1.0) / t2371;
    let t2373 = t216 * t2372;
    let t2382 = F::new(1.0) / t217 / t214;
    let t2386 = F::new(4.0) / F::new(9.0) * t2257;
    let t2394 = F::new(0.39862222222222222223e0) * t2257;
    let t2399 = F::new(1.0)/f64::sqrt(t214);
    let t2404 = F::new(0.13692777777777777778e0) * t2280;
    let t2414 = t777 * t777;
    (t2371, t2372, t2373, t2382, t2386, t2394, t2399, t2404, t2414)
}
