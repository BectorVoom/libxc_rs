//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1167/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1167(t11004: f64, t12567: f64, t3618: f64, t983: f64, t11002: f64, t3269: f64, t40491: f64, t986: f64, t3262: f64, t3263: f64, t11621: f64, t3275: f64, t40687: f64) -> (f64, f64, f64, f64) {
    let t42953 = 5.0_f64 / 16.0_f64 * t12567 * t11004;
    let t42955 = t3618 * t983;
    let t42956 = t11002 * t42955;
    let t42958 = 5.0_f64 / 8.0_f64 * t3269 * t42956;
    let t42959 = t40491 * t986;
    let t42962 = 3.0_f64 / 2.0_f64 * t3262 * t3263 * t42959;
    let t42965 = 45.0_f64 / 32.0_f64 * t3275 * t40687 * t11621;
    (t42953, t42958, t42962, t42965)
}
