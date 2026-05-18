//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1157/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1157<F: Float>(t1820: F, t1885: F, t40566: F, t995: F, t42175: F, t33298: F, t25514: F, t34565: F, t48373: F, t48377: F, t48380: F, t48381: F, t48382: F, t48387: F) -> (F, F, F, F) {
    let t48392 = F::new(16.0) / F::new(15.0) * t1820 * t1885 * t40566 * t995;
    let t48393 = F::new(64.0) / F::new(45.0) * t42175;
    let t48394 = F::new(32.0) / F::new(135.0) * t33298;
    let t48395 = t48373 + t48377 + t48380 - t48381 + t48382 + F::new(4.0) / F::new(45.0) * t34565 + t48387 - F::new(0.26596355555555555555e0) * t25514 - t48392 + t48393 - t48394;
    (t48392, t48393, t48394, t48395)
}
