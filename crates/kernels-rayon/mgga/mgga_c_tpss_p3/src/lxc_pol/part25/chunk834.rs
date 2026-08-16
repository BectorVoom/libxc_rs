//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 834/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk834(t5953: f64, t645: f64, t117: f64, t5815: f64, t1279: f64, t1281: f64, t1851: f64, t1853: f64, t547: f64, t548: f64, t5947: f64, t3418: f64, t38: f64) -> (f64, f64, f64, f64) {
    let t5954 = t5953 * t645;
    let t5957 = t117 * t5815;
    let t5960 = 3.0_f64 * t1279 * t1853 + 3.0_f64 * t1281 * t1851 + 6.0_f64 * t547 * t5954 + 3.0_f64 * t547 * t5957 + t548 * t5947;
    let t6073 = t3418 * t38;
    (t5954, t5957, t5960, t6073)
}
