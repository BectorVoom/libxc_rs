//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 779/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk779(t1592: f64, t7555: f64, t2832: f64, t537: f64, t255: f64, t571: f64, t1600: f64, t2631: f64, t551: f64, t6343: f64, t921: f64, t574: f64) -> (f64, f64, f64, f64) {
    let t7557 = 0.69345773920434148506e0_f64 * t1592 * t7555;
    let t7564 = t537 * t2832;
    let t7566 = t571 * t7564 * t255;
    let t7582 = 0.12805040077930161442e0_f64 * t1600 * t2631;
    let t7597 = t551 * t6343 * t921;
    let t7598 = t574 * t7597;
    (t7557, t7566, t7582, t7598)
}
