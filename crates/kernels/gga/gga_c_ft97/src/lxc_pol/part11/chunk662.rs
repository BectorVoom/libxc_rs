//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 662/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk662<F: Float>(t583: F, t8282: F, t462: F, t9178: F, t9179: F, t9181: F, t9183: F, t9186: F, t9188: F, t9190: F, t9193: F, t9196: F, t9199: F, t92: F) -> F {
    let t9202 = t8282 * t583;
    let t9204 = -t9178 - F::new(4.0) / F::new(3.0) * t9179 + t462 * t9181 + t462 * t9183 - t92 * t9186 - F::new(2.0) / F::new(3.0) * t9188 - F::new(2.0) / F::new(3.0) * t9190 + F::new(2.0) / F::new(3.0) * t462 * t9193 + F::new(4.0) / F::new(3.0) * t462 * t9196 - F::new(2.0) / F::new(3.0) * t462 * t9199 - F::new(4.0) / F::new(9.0) * t9202;
    t9204
}
