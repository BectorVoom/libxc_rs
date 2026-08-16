//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 876/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk876<F: Float>(t1593: F, t1655: F, t1710: F, t1712: F, t3020: F, t11360: F, t1602: F, t1685: F, t35: F, t428: F, t11240: F, t371: F) -> (F, F, F, F, F) {
    let t37960 = t1593 * t1655;
    let t37968 = t3020 * t1710 * t1712;
    let t37971 = t1602 * t11360;
    let t37977 = t35 * t1685;
    let t37978 = t37977 * t428;
    let t37985 = t371 * t11240;
    (t37960, t37968, t37971, t37978, t37985)
}
