//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 441/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk441<F: Float>(t6045: F, t6046: F, t173: F, t230: F, t1418: F, t1417: F, t6032: F, t8: F) -> (F, F, F, F, F) {
    let t6047 = t6045 * t6046;
    let t6050 = t173 * t230;
    let t6051 = t1418 * t6050;
    let t6053 = 0.6384360837962962963e-2 * t1417 * t6051;
    let t6054 = t6032 * t8;
    (t6047, t6050, t6051, t6053, t6054)
}
