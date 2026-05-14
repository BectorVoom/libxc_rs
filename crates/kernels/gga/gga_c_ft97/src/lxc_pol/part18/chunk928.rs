//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 928/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk928<F: Float>(t24130: F, t28: F, t1651: F, t1969: F, t5773: F, t1643: F, t9049: F, t1647: F, t2: F, t8787: F, t4: F, t26: F) -> (F, F, F, F, F, F) {
    let t24131 = t28 * t24130;
    let t24135 = t1969 * t5773 * t1651;
    let t24139 = t9049 * t5773 * t1643;
    let t24143 = t1969 * t5773 * t1647;
    let t24146 = t8787 * t2;
    let t24147 = t24146 * t4;
    let t24148 = t24147 * t26;
    (t24131, t24135, t24139, t24143, t24147, t24148)
}
