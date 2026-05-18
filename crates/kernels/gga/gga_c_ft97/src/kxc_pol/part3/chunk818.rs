//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 818/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk818<F: Float>(t16719: F, t9049: F, t446: F, t15742: F, t2205: F, t15737: F, t9327: F, t15746: F, t3281: F, t3408: F, t925: F, t1969: F) -> (F, F, F, F, F, F) {
    let t16720 = t9049 * t16719;
    let t16721 = t446 * t16720;
    let t16723 = t2205 * t15742;
    let t16724 = t446 * t16723;
    let t16726 = t9327 * t15737;
    let t16727 = t446 * t16726;
    let t16729 = t2205 * t15746;
    let t16730 = t3281 * t16729;
    let t16732 = t925 * t3408;
    let t16733 = t1969 * t16732;
    (t16721, t16724, t16727, t16730, t16732, t16733)
}
