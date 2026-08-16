//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2460/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2460<F: Float>(t69003: F, t69005: F, t69011: F, t69014: F, t69018: F, t69021: F, t69023: F, t69025: F, t69027: F, t69030: F, t69036: F, t69253: F, t69255: F, t69257: F, t69259: F, t69261: F, t69453: F, t69456: F, t69459: F, t69461: F) -> F {
    let t69961 = -t69003 + t69005 - t69011 - t69014 + t69018 + t69021 - t69023 + t69025 - t69027 - t69030 + t69036 + t69453 + t69456 + t69459 + t69461 - t69253 + t69255 - t69257 + t69259 + t69261;
    t69961
}
