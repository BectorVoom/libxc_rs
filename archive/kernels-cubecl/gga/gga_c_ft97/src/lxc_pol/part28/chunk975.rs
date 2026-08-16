//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 975/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk975<F: Float>(t1349: F, t32691: F, t376: F, t1637: F, t7341: F, t32686: F, t5769: F, t24116: F, t7309: F, t7345: F, t32685: F, t92: F) -> (F, F, F, F, F, F) {
    let t138655 = t1349 * t376 * t32691;
    let t138662 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t1349 * t1637 * t7341;
    let t138677 = t32686 * t5769;
    let t138681 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t7309 * t24116;
    let t138705 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t1349 * t1637 * t7345;
    let t138706 = t32685 * t92;
    (t138655, t138662, t138677, t138681, t138705, t138706)
}
