//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 726/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk726<F: Float>(t473: F, t7165: F, t7243: F, t32057: F, t7239: F, t174: F, t626: F) -> (F, F, F, F) {
    let t32058 = t7165 * t473;
    let t32059 = t7243 * t32058;
    let t32061 = t32057 * t7239 * t32059;
    let t32063 = t626 * t174;
    (t32058, t32059, t32061, t32063)
}
