//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 586/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk586<F: Float>(t26: F, t7943: F, t1771: F, t380: F, t17: F, t7760: F, t62: F, t66: F, t401: F, t77: F, t408: F, t428: F, t3020: F, t1609: F, t1593: F, t1608: F) -> (F, F, F, F, F, F, F) {
    let t7944 = t26 * t7943;
    let t7945 = 28.0 / 27.0 * t7944;
    let t7946 = t1771 * t380;
    let t7954 = t17 * t7760;
    let t7983 = t62 * t66;
    let t7984 = t77 * t401;
    let t7985 = t7983 * t7984;
    let t7988 = t408 * t428;
    let t7989 = t3020 * t7988;
    let t8007 = t77 * t1609;
    let t8008 = t8007 * t1593;
    let t8009 = t1608 * t8008;
    (t7944, t7945, t7946, t7954, t7985, t7989, t8009)
}
