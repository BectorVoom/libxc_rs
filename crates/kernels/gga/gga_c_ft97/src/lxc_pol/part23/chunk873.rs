//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 873/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk873<F: Float>(t213: F, t703: F, t684: F, t27659: F, t3751: F, t6036: F, t2383: F, t695: F) -> (F, F, F, F, F) {
    let t27660 = t703 * t213;
    let t27661 = t27660 * t684;
    let t27662 = t27659 * t27661;
    let t27665 = t6036 * t3751;
    let t27669 = t2383 * t695;
    (t27660, t27661, t27662, t27665, t27669)
}
