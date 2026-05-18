//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 536/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk536<F: Float>(t5995: F, t92: F, t1426: F, t2399: F, t6067: F, t681: F, t6063: F, t6008: F, t683: F) -> (F, F, F, F, F) {
    let t24204 = t5995 * t92;
    let t24211 = t2399 * t1426;
    let t24220 = t681 * t6067;
    let t24223 = t681 * t6063;
    let t24231 = t683 * t6008;
    (t24204, t24211, t24220, t24223, t24231)
}
