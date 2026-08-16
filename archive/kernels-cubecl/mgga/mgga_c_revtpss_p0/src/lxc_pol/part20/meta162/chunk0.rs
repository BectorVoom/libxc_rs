//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 872/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk872<F: Float>(t198: F, t532: F, t539: F, t73: F, t241: F, t4000: F, t820: F, t550: F, t72: F, t245: F) -> (F, F, F, F) {
    let t5541 = t198 * t532;
    let t5650 = t539 * t73;
    let t5671 = t820 * t4000 * t241;
    let t5672 = t550 * t72;
    let t5673 = t5672 * t245;
    (t5541, t5650, t5671, t5673)
}
