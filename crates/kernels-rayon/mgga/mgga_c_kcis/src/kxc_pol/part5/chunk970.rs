//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 970/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk970(t1164: f64, t3225: f64, t334: f64, t369: f64, t86: f64, t1143: f64, t245: f64, t1157: f64, t752: f64, t1071: f64, t1083: f64, t2844: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10525 = t1164 * t3225;
    let t10526 = t10525 * sigma0;
    let t10541 = 0.11791604938271604938e-1_f64 * t86 * t334 * t369;
    let t10544 = t1143 * t245;
    let t10556 = t752 * t1157;
    let t10560 = t1083 * t1071;
    let t10583 = t1083 * t2844;
    (t10526, t10541, t10544, t10556, t10560, t10583)
}
