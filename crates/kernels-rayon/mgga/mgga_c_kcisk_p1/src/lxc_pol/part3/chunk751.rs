//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 751/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk751(t10488: f64, t1659: f64, t1835: f64, t1060: f64, t1846: f64, t3293: f64, t696: f64, t11578: f64, t11580: f64, t11583: f64, t11586: f64, t11588: f64, t11590: f64, t11593: f64, t11596: f64, t158: f64, t165: f64, t173: f64) -> f64 {
    let t11599 = t1659 * t10488;
    let t11602 = t1835 * t10488;
    let t11605 = t1846 * t1060;
    let t11607 = t696 * t3293;
    let t11609 = -0.4684e-2_f64 * t11578 - 0.3513e-2_f64 * t158 * t11580 + 0.78066666666666666667e-3_f64 * t158 * t11583 - 0.39624999999999999999e-2_f64 * t11586 + 0.26416666666666666666e-2_f64 * t11588 + 0.7925e-3_f64 * t165 * t11590 - 0.17611111111111111111e-3_f64 * t165 * t11593 - 0.7026e-2_f64 * t158 * t11596 + 0.317e-2_f64 * t165 * t11599 + 0.403305e-4_f64 * t173 * t11602 + 0.71734315950379065738e-1_f64 * t11605 - 0.35867157975189532869e-1_f64 * t11607;
    t11609
}
