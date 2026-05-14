//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 571/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk571<F: Float>(t157: F, t9114: F, t1557: F, t604: F, t1570: F, t1984: F, t355: F) -> (F, F, F, F) {
    let t9115 = t9114 * t157;
    let t9121 = t604 * t1557;
    let t9127 = t604 * t1570;
    let t9132 = t355 * t1984;
    (t9115, t9121, t9127, t9132)
}
