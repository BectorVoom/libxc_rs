//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1238/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1238(t11476: f64, t40282: f64, t11560: f64, t40713: f64, t42846: f64, t481: f64, t37327: f64, t4176: f64, t35373: f64, t795: f64, t14160: f64, t40630: f64) -> (f64, f64, f64, f64) {
    let t43764 = 3.0_f64 / 2.0_f64 * t40282 * t11476;
    let t43766 = 5.0_f64 / 8.0_f64 * t40713 * t11560;
    let t43767 = t42846 * t481;
    let t43770 = 15.0_f64 / 8.0_f64 * t37327 * t4176 * t43767;
    let t43771 = t35373 * t795;
    let t43774 = 3.0_f64 * t40630 * t14160 * t43771;
    (t43764, t43766, t43770, t43774)
}
