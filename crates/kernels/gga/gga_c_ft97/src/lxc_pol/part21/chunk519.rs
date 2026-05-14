//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 519/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk519<F: Float>(t5778: F, t6587: F, t28: F, t1008: F, t1701: F, t5546: F, t1013: F, t72: F) -> (F, F, F, F, F) {
    let t6588 = t5778 * t6587;
    let t6589 = t28 * t6588;
    let t6593 = t1701 * t5546 * t1008;
    let t6597 = t1701 * t5546 * t1013;
    let t6604 = t72 * t1008;
    (t6588, t6589, t6593, t6597, t6604)
}
