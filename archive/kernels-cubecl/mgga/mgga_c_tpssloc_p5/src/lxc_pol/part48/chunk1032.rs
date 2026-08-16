//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 1032/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk1032<F: Float>(t112: F, t32392: F, t111: F, t8843: F, t114483: F, t114489: F, t114494: F, t114500: F, t115990: F, t115995: F, t116000: F, t116004: F, t116006: F, t116008: F, t117662: F, t2319: F, t23917: F, t24478: F, t24481: F, t24969: F, t24972: F, t577: F, t671: F, t7056: F, t7235: F, t7423: F, t85416: F) -> F {
    let t117672 = t32392 * t112;
    let t117687 = t8843 * t111;
    let t117690 = F::cast_from(27.0_f64) * t117672 * t671 + F::cast_from(0.135e2_f64) * t7423 * t23917 + t115990 + t114483 + t114489 + t115995 + t114494 + F::cast_from(54.0_f64) * t85416 * t7235 + t116000 + F::cast_from(27.0_f64) * t24969 * t7056 + t114500 + F::cast_from(0.45e1_f64) * t117662 * t577 + F::cast_from(54.0_f64) * t24972 * t24478 + F::cast_from(27.0_f64) * t24972 * t24481 + F::cast_from(27.0_f64) * t117687 * t2319 + t116004 + t116006 + t116008;
    t117690
}
