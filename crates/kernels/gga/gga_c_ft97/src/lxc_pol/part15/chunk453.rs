//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 453/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk453<F: Float>(t108: F, t4415: F, t4501: F, t4552: F, t4590: F, t4594: F, t4621: F, t4623: F, t88: F, t948: F, t984: F, t4431: F) -> (F, F) {
    let t4628 = -t108 * t4415 - t108 * t4501 - t4621 * t88 - F::new(2.0) * t948 * t984 + F::new(4.0) * t4552 - F::new(2.0) * t4590 - F::new(4.0) * t4594 + F::new(2.0) * t4623;
    let t4635 = -t4431;
    (t4628, t4635)
}
