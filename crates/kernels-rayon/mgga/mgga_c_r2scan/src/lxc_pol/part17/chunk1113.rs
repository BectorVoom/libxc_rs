//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1113/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1113(t38145: f64, t6085: f64, t8081: f64, t6093: f64, t7619: f64, t2147: f64, t7624: f64, t1575: f64, t269: f64, t546: f64, t565: f64, t10728: f64, t7258: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40047 = t6085 * t38145 * t8081;
    let t40050 = t6093 * t38145 * t7619;
    let t40053 = t2147 * t38145 * t7624;
    let t40061 = t1575 * t269;
    let t40062 = t546 * t40061;
    let t40066 = t565 * t40061;
    let t40070 = t10728 * t7258;
    (t40047, t40050, t40053, t40062, t40066, t40070)
}
