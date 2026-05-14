//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 636/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk636<F: Float>(t1045: F, t2101: F, t1055: F, t8232: F, t1882: F, t3548: F, t3575: F, t1030: F, t167: F, t9114: F, t2179: F, t582: F, t3596: F, t5: F, t1775: F, t3918: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t13153 = t2101 * t1045;
    let t13187 = t8232 * t1055;
    let t13190 = 2.0 / 9.0 * t1882 * t3548;
    let t13196 = 2.0 / 9.0 * t1882 * t3575;
    let t13201 = t8232 * t1030;
    let t13208 = t2101 * t167;
    let t13212 = t9114 * t167;
    let t13220 = t582 * t2179;
    let t13273 = t5 * t3596;
    let t13306 = 4.0 / 9.0 * t1775 * t3918;
    (t13153, t13187, t13190, t13196, t13201, t13208, t13212, t13220, t13273, t13306)
}
