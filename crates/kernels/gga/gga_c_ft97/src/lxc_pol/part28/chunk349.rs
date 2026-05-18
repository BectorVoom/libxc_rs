//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 349/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk349<F: Float>(t5570: F, t5572: F, t5566: F, t8: F, t1669: F, t12: F, t47: F, t14: F) -> (F, F, F, F, F) {
    let t5573 = t5570 * t5572;
    let t5576 = t5566 * t8;
    let t5577 = t1669 * t5576;
    let t5578 = t12 * t47;
    let t5579 = t5578 * t14;
    (t5573, t5576, t5577, t5578, t5579)
}
