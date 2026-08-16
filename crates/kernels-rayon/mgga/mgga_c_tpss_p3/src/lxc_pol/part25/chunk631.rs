//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 631/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk631(t1425: f64, t2476: f64, t865: f64, t2481: f64, t1415: f64, t2487: f64, t849: f64, t2455: f64, t2491: f64, t3746: f64, t3751: f64, t3756: f64, t3760: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3769 = 1.0_f64 * t2476 * t1425;
    let t3770 = t1425 * t865;
    let t3772 = 2.0_f64 * t2481 * t3770;
    let t3773 = t2487 * t1415;
    let t3774 = t3773 * t849;
    let t3781 = t2491 + t2455 / 9.0_f64 + t3746 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t3751 + 2.0_f64 / 3.0_f64 * t3756 - t3760 / 3.0_f64;
    (t3769, t3770, t3772, t3773, t3774, t3781)
}
