//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 644/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk644<F: Float>(t3057: F, t6: F, t39: F, t930: F, t3056: F, t77: F, t3020: F, t122: F, t938: F, t1593: F, t1595: F, t35: F, t1655: F, t929: F, t1594: F, t11084: F, t534: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11127 = t3057 * t6;
    let t11131 = t930 * t39;
    let t11135 = t77 * t3056;
    let t11136 = t3020 * t11135;
    let t11139 = t938 * t122;
    let t11140 = t1593 * t1595;
    let t11141 = t11140 * t35;
    let t11142 = t11139 * t11141;
    let t11145 = t929 * t1655;
    let t11146 = t11145 * t35;
    let t11147 = t1594 * t11146;
    let t11150 = t534 * t11084;
    (t11127, t11131, t11136, t11141, t11142, t11145, t11146, t11147, t11150)
}
