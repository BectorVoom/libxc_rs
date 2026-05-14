//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 876/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk876<F: Float>(t3578: F, t5968: F, t2142: F, t6718: F, t1009: F, t5790: F, t12374: F, t5812: F, t3379: F, t72: F, t5579: F, t1013: F, t53: F, t5591: F, t1008: F, t2035: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t26597 = t3578 * t5968;
    let t26599 = t2142 * t6718;
    let t26601 = t5790 * t1009;
    let t26604 = t12374 * t5812;
    let t26607 = t72 * t3379;
    let t26608 = t5579 * t26607;
    let t26611 = t1013 * t53;
    let t26612 = t72 * t26611;
    let t26613 = t5591 * t26612;
    let t26617 = t2035 * t5790 * t1008;
    (t26597, t26599, t26601, t26604, t26607, t26608, t26611, t26612, t26613, t26617)
}
