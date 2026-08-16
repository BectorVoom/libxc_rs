//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 867/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk867(t1187: f64, t3523: f64, t5205: f64, t1196: f64, t3358: f64, t3546: f64, t5044: f64, t5049: f64, t5054: f64, t5058: f64, t459: f64) -> (f64, f64, f64, f64, f64) {
    let t5206 = t3523 * t1187;
    let t5207 = t5205 * t5206;
    let t5209 = 0.17315859105681463759e2_f64 * t1196 * t5207;
    let t5215 = t3546 - 0.27777777777777777778e-2_f64 * t3358 - 0.27777777777777777778e-2_f64 * t5044 - 0.55555555555555555555e-2_f64 * t5049 + 0.16666666666666666667e-1_f64 * t5054 + 0.83333333333333333333e-2_f64 * t5058;
    let t5216 = t5215 * t459;
    (t5206, t5207, t5209, t5215, t5216)
}
