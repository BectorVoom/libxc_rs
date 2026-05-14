//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1204/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1204<F: Float>(t16937: F, t29357: F, t7908: F, t1386: F, t7086: F, t101928: F, t101931: F, t101934: F, t101938: F, t101941: F, t101948: F, t101954: F, t101957: F, t1380: F, t28348: F, t28353: F, t28372: F, t98119: F) -> (F, F) {
    let t103141 = t7908 * t16937 * t29357;
    let t103149 = t1386 * t7086;
    let t103154 = 0.17687407407407407407e-1 * t101928 - 0.14739506172839506172e-1 * t101931 + 0.22109259259259259259e-2 * t101934 - 0.66327777777777777776e-2 * t101938 + 0.55273148148148148147e-2 * t101941 - 0.33163888888888888888e-2 * t101948 - 0.7722800925925925926e-4 * t103141 - 0.55652820312500000001e-3 * t98119 * t28353 - 0.18550940104166666667e-3 * t98119 * t28348 - 0.33163888888888888888e-2 * t101954 - 0.11054629629629629629e-2 * t101957 - 0.13901041666666666667e-2 * t7908 * t28372 * t103149 * t1380;
    (t103149, t103154)
}
