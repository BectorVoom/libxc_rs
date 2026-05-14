//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1228/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1228<F: Float>(t1111: F, t26143: F, t27048: F, t27053: F, t27056: F, t27061: F, t27063: F, t27067: F, t27074: F, t27076: F, t27079: F, t27084: F, t27088: F, t27093: F, t3103: F, t3116: F, t3132: F, t322: F, t4357: F, t8469: F) -> (F,) {
    let t27095 = -0.18314556960919660338e2 * t3132 * t27048 * t4357 - t27053 / 162.0 - t27056 / 27.0 - t27061 + 0.36629113921839320676e2 * t3103 * t8469 * t27063 + 0.5680050638253047068e0 * t3116 * t27067 * t26143 - t27074 + 5.0 / 972.0 * t27076 + 7.0 / 486.0 * t27079 + 35.0 / 972.0 * t1111 * t322 * t27084 + t1111 * t322 * t27088 / 288.0 - t27093 / 216.0;
    (t27095,)
}
