//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 874/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk874<F: Float>(t2213: F, t23470: F, t2157: F, t5935: F, t144: F, t5929: F, t604: F) -> (F, F, F, F) {
    let t23471 = t23470 * t2213;
    let t23474 = t5935 * t2157;
    let t23475 = t144 * t23474;
    let t23478 = t5929 * t604;
    (t23471, t23474, t23475, t23478)
}
