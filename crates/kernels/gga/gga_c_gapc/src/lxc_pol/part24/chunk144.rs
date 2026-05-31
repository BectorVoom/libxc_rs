//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 144/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk144<F: Float>(t481: F, t6: F, t134: F, t128: F, t137: F, t139: F, t124: F, t193: F, t5: F) -> (F, F, F, F, F, F, F) {
    let t482 = t6 * t481;
    let t483 = t482 * t134;
    let t486 = t482 * t128;
    let t487 = t137 * t139;
    let t488 = t487 * t124;
    let t491 = F::cast_from(1.0_f64) / t193;
    let t492 = t5 * t491;
    (t482, t483, t486, t487, t488, t491, t492)
}
