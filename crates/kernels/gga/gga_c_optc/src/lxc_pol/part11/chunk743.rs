//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 743/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk743<F: Float>(t496: F, t8545: F, t492: F, t490: F, t2910: F, t474: F, t8639: F, t8642: F, t1198: F, t481: F, t1998: F, t3386: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t9226 = t8545 * t496;
    let t9227 = t492 * t9226;
    let t9229 = F::new(5.0) / F::new(27.0) * t490 * t9227;
    let t9254 = F::new(1.0) / t2910 / t474;
    let t9268 = F::cast_from(0.60319259259259259259e1_f64) * t8639;
    let t9269 = F::cast_from(0.54733333333333333333e-2_f64) * t8642;
    let t9302 = t1198 * t1198;
    let t9303 = F::new(1.0) / t9302;
    let t9304 = t481 * t9303;
    let t9311 = F::cast_from(0.22615185185185185185e4_f64) * t8639;
    let t9312 = F::cast_from(0.34962962962962962963e3_f64) * t8642;
    let t9392 = t3386 * t1998;
    (t9226, t9227, t9229, t9254, t9268, t9269, t9302, t9303, t9304, t9311, t9312, t9392)
}
