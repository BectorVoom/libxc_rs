//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 837/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk837(t4712: f64, t5106: f64, t671: f64, t88: f64, t1268: f64, t1458: f64, t2314: f64, t4026: f64, t4028: f64, t4072: f64, t1390: f64, t1845: f64) -> (f64, f64, f64, f64) {
    let t5107 = t4712 + t5106;
    let t5113 = t88 * t671;
    let t5118 = 2.0_f64 * t1268 * t4072 + 2.0_f64 * t1458 * t2314 + 2.0_f64 * t1458 * t5113 + 2.0_f64 * t4028 * t671 + t4026;
    let t5122 = t1845 * t1390;
    (t5107, t5113, t5118, t5122)
}
