//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 574/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk574<F: Float>(t3020: F, t3070: F, t1771: F, t926: F, t14: F, t7741: F, t12: F) -> (F, F, F, F) {
    let t11160 = t3020 * t3070;
    let t11167 = t1771 * t926;
    let t11174 = 1.0 / t14 / t7741;
    let t11175 = t12 * t11174;
    (t11160, t11167, t11174, t11175)
}
