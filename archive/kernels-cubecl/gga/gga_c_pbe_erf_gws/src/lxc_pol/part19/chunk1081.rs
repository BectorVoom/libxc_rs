//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1081/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1081<F: Float>(t11602: F, t11604: F, t11613: F, t11632: F, t11646: F, t11650: F, t11665: F, t11670: F, t11695: F, t11699: F, t8901: F, t8927: F, t8960: F) -> F {
    let t12152 = -t11602 - t8901 - t11604 + t11613 - t11632 - t8927 + t11646 - t11650 + t11665 - t11670 + t8960 - t11695 + t11699;
    t12152
}
