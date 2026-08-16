//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 500/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk500(t20: f64, t2314: f64, t21: f64, t6: f64, t736: f64, t649: f64, t66: f64, t648: f64, t119: f64, t5: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2315 = t2314 * t20;
    let t2316 = t21 * t6;
    let t2317 = t2316 * t736;
    let t2318 = t2315 * t2317;
    let t2320 = t649 * t66;
    let t2321 = t648 * t2320;
    let t2323 = t5 * t119;
    (t2315, t2316, t2317, t2318, t2320, t2321, t2323)
}
