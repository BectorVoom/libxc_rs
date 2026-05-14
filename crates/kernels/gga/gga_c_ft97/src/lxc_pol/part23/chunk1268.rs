//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1268/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1268<F: Float>(t122702: F, t123833: F, t123855: F, t123903: F, t123939: F, t123977: F, t124018: F, t124047: F, t124089: F, t124123: F, t124150: F, t124180: F, t124221: F, t124264: F, t124309: F, t124327: F) -> (F,) {
    let t124331 = t122702 + t123833 + t123855 + t123903 + t123939 + t123977 + t124018 + t124047 + t124089 + t124123 + t124150 + t124180 + t124221 + t124264 + t124309 + t124327;
    (t124331,)
}
