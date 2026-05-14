//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 455/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk455<F: Float>(t1690: F, t5552: F, t5555: F, t428: F, t5546: F, t1701: F, t408: F, t6: F) -> (F, F, F) {
    let t5557 = t1690 * t5552 * t5555;
    let t5560 = t5546 * t428;
    let t5561 = t1701 * t5560;
    let t5566 = t408 * t6;
    (t5557, t5561, t5566)
}
