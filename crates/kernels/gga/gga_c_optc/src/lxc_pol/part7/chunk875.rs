//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 875/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk875<F: Float>(t43: F, t7249: F, t7321: F, t8303: F, t8397: F, t6541: F, t176: F, t2902: F, t1219: F, t2848: F, t50: F, zeta_threshold: F) -> (F, F, F, F) {
    let t44 = t43 <= zeta_threshold;
    let t8399 = t7249 + t7321 + t8303 + t8397;
    let t8406 = piecewise3::<F>(t44, F::new(0.0), t6541);
    let t8409 = t176 * t2902;
    let t8410 = t8409 * t1219;
    let t8414 = F::new(1.0) / t2848 / t50;
    (t8399, t8406, t8410, t8414)
}
