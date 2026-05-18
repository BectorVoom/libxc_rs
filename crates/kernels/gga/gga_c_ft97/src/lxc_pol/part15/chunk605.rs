//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 605/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk605<F: Float>(t11175: F, t9: F, t371: F, t7876: F, t1630: F, t929: F, t173: F, t1736: F, t420: F, t8119: F, t626: F, t934: F) -> (F, F, F, F, F, F) {
    let t11176 = t9 * t11175;
    let t11232 = t371 * t7876;
    let t11233 = t1630 * t929;
    let t11262 = t173 * t1736;
    let t11269 = t420 * t8119;
    let t11298 = t626 * t934;
    (t11176, t11232, t11233, t11262, t11269, t11298)
}
