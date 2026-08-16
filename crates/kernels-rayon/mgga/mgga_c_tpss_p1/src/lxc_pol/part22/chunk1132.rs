//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1132/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1132(t1586: f64, t3118: f64, t3144: f64, t3053: f64, t9751: f64, t9765: f64, t1561: f64, t3110: f64, t1133: f64, t4245: f64, t12352: f64, t466: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12577 = t3118 * t1586 * t3144;
    let t12580 = t9751 * t3053;
    let t12590 = t9765 * t3053;
    let t12597 = t3110 * t1561;
    let t12600 = t1133 * t4245;
    let t12607 = t466 * t12352;
    (t12577, t12580, t12590, t12597, t12600, t12607)
}
