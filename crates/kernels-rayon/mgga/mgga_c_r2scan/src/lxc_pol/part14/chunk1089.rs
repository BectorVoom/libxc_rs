//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1089/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1089(t23194: f64, t263: f64, t3438: f64, t3446: f64, t6874: f64, t10968: f64, t6262: f64, t6855: f64, t10930: f64, t158: f64, t2304: f64, t2317: f64, t3434: f64) -> (f64, f64, f64) {
    let t38225 = t3446 * t263 * t23194 * t3438 * t6874;
    let t38228 = t6855 * t6262 * t10968;
    let t38233 = t3434 * t2304 * t2317 * t158 * t10930;
    (t38225, t38228, t38233)
}
