//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 707/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk707<F: Float>(t11139: F, t11141: F, t1655: F, t929: F, t35: F, t1594: F, t11084: F, t534: F, t1595: F, t1630: F, t3064: F, t3020: F, t3070: F) -> (F, F, F, F, F, F, F) {
    let t11142 = t11139 * t11141;
    let t11145 = t929 * t1655;
    let t11146 = t11145 * t35;
    let t11147 = t1594 * t11146;
    let t11150 = t534 * t11084;
    let t11153 = t1630 * t1595;
    let t11154 = t11153 * t35;
    let t11155 = t3064 * t11154;
    let t11160 = t3020 * t3070;
    (t11142, t11145, t11146, t11147, t11150, t11155, t11160)
}
