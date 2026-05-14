//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 796/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk796<F: Float>(t378: F, t38057: F, t92: F, t11401: F, t23: F, t26: F, t37357: F, t37406: F, t7954: F, t37311: F, t7763: F, t1642: F, t38042: F, t38044: F, t38046: F, t38048: F, t38050: F, t38055: F) -> (F, F, F, F, F, F, F, F, F) {
    let t38059 = t92 * t378 * t38057;
    let t38061 = t11401 * t23;
    let t38062 = t26 * t38061;
    let t38063 = 280.0 / 81.0 * t38062;
    let t38064 = t37406 * t37357;
    let t38066 = t92 * t7954 * t38064;
    let t38069 = t92 * t7954 * t37311;
    let t38071 = t7763 * t37357;
    let t38073 = t92 * t1642 * t38071;
    let t38075 = 16.0 / 9.0 * t38042 - 16.0 / 9.0 * t38044 + 8.0 / 9.0 * t38046 + 8.0 / 3.0 * t38048 - 8.0 / 3.0 * t38050 - 80.0 / 81.0 * t38055 - t38059 / 3.0 + t38063 + 40.0 / 9.0 * t38066 - 20.0 / 9.0 * t38069 - 8.0 * t38073;
    (t38059, t38061, t38062, t38064, t38066, t38069, t38071, t38073, t38075)
}
