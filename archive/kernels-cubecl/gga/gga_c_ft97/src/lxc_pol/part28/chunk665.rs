//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 665/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk665<F: Float>(t3379: F, t72: F, t5579: F, t1013: F, t53: F, t5591: F, t1008: F, t2035: F, t5790: F, t1701: F, t3404: F, t5546: F) -> (F, F, F, F) {
    let t26607 = t72 * t3379;
    let t26608 = t5579 * t26607;
    let t26611 = t1013 * t53;
    let t26612 = t72 * t26611;
    let t26613 = t5591 * t26612;
    let t26617 = t2035 * t5790 * t1008;
    let t26621 = t1701 * t5546 * t3404;
    (t26608, t26613, t26617, t26621)
}
