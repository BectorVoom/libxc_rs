//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 856/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk856<F: Float>(t11717: F, t3922: F, t3936: F, t458: F, t2349: F, t3690: F) -> (F, F, F) {
    let t13339 = t11717 * t3922;
    let t13345 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t458 * t3936;
    let t13346 = t3690 * t2349;
    (t13339, t13345, t13346)
}
