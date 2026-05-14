//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 362/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk362<F: Float>(t2211: F, t2212: F, t2210: F, t1647: F, t167: F, t569: F, t157: F, t2101: F) -> (F, F, F, F) {
    let t2213 = t2211 * t2212;
    let t2214 = t2210 * t2213;
    let t2218 = t569 * t167 * t1647;
    let t2221 = t2101 * t157;
    (t2213, t2214, t2218, t2221)
}
