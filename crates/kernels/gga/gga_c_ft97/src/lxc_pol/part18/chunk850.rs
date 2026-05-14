//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 850/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk850<F: Float>(t23110: F, t23127: F, t103: F, t2: F, t7733: F, t4: F, t26: F) -> (F, F, F, F) {
    let t23128 = t23110 + t23127;
    let t23129 = t23128 * t103;
    let t23131 = t7733 * t2;
    let t23132 = t23131 * t4;
    let t23133 = t23132 * t26;
    (t23128, t23129, t23132, t23133)
}
