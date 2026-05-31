//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1793/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1793<F: Float>(t40067: F, t40072: F, t4140: F, t47100: F, t47102: F, t47107: F, t47109: F, t47111: F, t47114: F, t47116: F, t47118: F, t47120: F, t47122: F, t47124: F, t47126: F, t5536: F, t9984: F) -> F {
    let t47681 = F::cast_from(72.0_f64) * t4140 * t5536 * t9984 + t40067 - t40072 - t47100 - t47102 - t47107 - t47109 - t47111 + t47114 + t47116 - t47118 - t47120 + t47122 + t47124 + t47126;
    t47681
}
