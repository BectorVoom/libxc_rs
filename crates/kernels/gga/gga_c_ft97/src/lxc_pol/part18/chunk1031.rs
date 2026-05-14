//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1031/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1031<F: Float>(t1039: F, t590: F, t574: F, t5900: F, t27157: F, t23649: F, t6662: F, t2: F, t6615: F) -> (F, F, F, F, F) {
    let t27158 = t1039 * t590;
    let t27160 = t574 * t5900 * t27158;
    let t27161 = t27157 * t27160;
    let t27163 = t23649 * t6662;
    let t27165 = t2 * t6615;
    (t27158, t27160, t27161, t27163, t27165)
}
