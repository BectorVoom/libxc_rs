//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1176/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1176(t1133: f64, t6613: f64, t1131: f64, t1096: f64, t1092: f64, t3211: f64, t6276: f64, t3210: f64, t3200: f64, t19552: f64, t9512: f64, t4554: f64) -> (f64, f64, f64, f64) {
    let t19735 = t6613 * t1133;
    let t19736 = t1131 * t19735;
    let t19737 = t1096 * t19736;
    let t19738 = t1092 * t19737;
    let t19741 = t3211 * t6276 * t1133;
    let t19742 = t3210 * t19741;
    let t19743 = t3200 * t19742;
    let t19745 = t9512 * t19552;
    let t19746 = t3210 * t19745;
    let t19747 = t4554 * t19746;
    (t19735, t19738, t19743, t19747)
}
