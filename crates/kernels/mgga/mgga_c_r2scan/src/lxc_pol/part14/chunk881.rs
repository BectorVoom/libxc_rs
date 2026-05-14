//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 881/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk881<F: Float>(t11056: F, t819: F, t10687: F, t3275: F, t3465: F, t10610: F, t10611: F, t1114: F, t5086: F) -> (F, F, F, F) {
    let t11057 = t819 * t11056;
    let t11184 = t3275 * t3465 * t10687;
    let t11185 = t11184 / 4.0;
    let t11187 = t10610 * t3465 * t10611;
    let t11188 = 3.0 / 2.0 * t11187;
    let t11189 = t5086 * t1114;
    (t11057, t11185, t11188, t11189)
}
