//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 838/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk838<F: Float>(t24330: F, t6249: F, t6250: F, t1472: F, t24287: F, t1476: F, t2360: F) -> (F, F, F) {
    let t25118 = t6249 * t24330 * t6250;
    let t25132 = 0.11113000182098765433e-1 * t1472 * t24287;
    let t25140 = t1476 * t2360;
    (t25118, t25132, t25140)
}
