//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 460/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk460<F: Float>(t2058: F, t4702: F, t133: F, t2066: F, t3086: F, t4481: F, t4485: F, t4489: F) -> (F, F, F) {
    let t4703 = t2058 * t4702;
    let t4704 = t133 * t4703;
    let t4710 = -t2066 + F::new(0.11113000182098765433e-1) * t3086 + F::new(0.22226000364197530865e-1) * t4481 - F::new(0.33339000546296296298e-1) * t4485 + F::new(0.16669500273148148149e-1) * t4489;
    (t4703, t4704, t4710)
}
