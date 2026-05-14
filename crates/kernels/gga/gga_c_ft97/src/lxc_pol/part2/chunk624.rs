//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 624/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk624<F: Float>(t10992: F, t3052: F, t432: F, t1564: F, t3281: F, t1580: F, t2992: F) -> (F, F, F, F) {
    let t10993 = t10992 / 27.0;
    let t10994 = t3052 * t432;
    let t10995 = t1564 * t10994;
    let t10996 = t3281 * t10995;
    let t10998 = t2992 * t1580;
    (t10993, t10994, t10996, t10998)
}
