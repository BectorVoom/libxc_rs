//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1125/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1125<F: Float>(t24046: F, t604: F, t2: F, t23884: F, t23649: F, t23654: F, t458: F, t5889: F) -> (F, F, F, F) {
    let t95026 = t24046 * t604;
    let t95029 = t2 * t23884;
    let t95051 = t23649 * t23654;
    let t95053 = t5889 * t458;
    (t95026, t95029, t95051, t95053)
}
