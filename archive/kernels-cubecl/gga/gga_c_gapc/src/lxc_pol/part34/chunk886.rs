//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 886/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk886<F: Float>(t10023: F, t10082: F, t9428: F, t9466: F, t9511: F, t9548: F, t9594: F, t9633: F, t9676: F, t9726: F, t9773: F, t9813: F, t9855: F, t9891: F, t9943: F, t9983: F) -> F {
    let t10086 = t9428 + t9466 + t9511 + t9548 + t9594 + t9633 + t9676 + t9726 + t9773 + t9813 + t9855 + t9891 + t9943 + t9983 + t10023 + t10082;
    t10086
}
