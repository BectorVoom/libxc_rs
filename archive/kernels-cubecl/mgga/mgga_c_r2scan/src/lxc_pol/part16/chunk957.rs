//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 957/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk957<F: Float>(t11518: F, t3276: F, t3262: F, t106: F, t2530: F, t97: F, t3271: F, t10619: F, t3579: F, t10615: F, t3275: F, t3582: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11519 = t3276 * t11518;
    let t11520 = t3262 * t11519;
    let t11521 = F::cast_from(15.0_f64) / F::cast_from(16.0_f64) * t11520;
    let t11523 = t97 * t106 * t2530;
    let t11524 = t11523 * t3271;
    let t11525 = t11524 / F::cast_from(4.0_f64);
    let t11526 = t3579 * t10619;
    let t11527 = t11526 / F::cast_from(4.0_f64);
    let t11529 = t3275 * t10615 * t3582;
    (t11519, t11520, t11521, t11523, t11524, t11525, t11526, t11527, t11529)
}
