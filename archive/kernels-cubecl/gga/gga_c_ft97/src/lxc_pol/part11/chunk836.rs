//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 836/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk836<F: Float>(t1882: F, t7826: F, t369: F, t7954: F, t432: F, t7955: F, t446: F, t1559: F, t1580: F, t7763: F) -> (F, F, F, F) {
    let t37303 = t1882 * t7826;
    let t37305 = t7954 * t369;
    let t37306 = t7955 * t432;
    let t37308 = t446 * t37305 * t37306;
    let t37311 = t7763 * t1559 * t1580;
    (t37303, t37306, t37308, t37311)
}
