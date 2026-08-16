//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1260/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1260(t1020: f64, t1129: f64, t1131: f64, t12285: f64, t12298: f64, t12300: f64, t12302: f64, t12894: f64, t2410: f64, t2956: f64, t3522: f64, t3524: f64, t3526: f64, t3530: f64, t3745: f64, t3749: f64, t3753: f64, t3757: f64, t839: f64, t9707: f64) -> f64 {
    let t44778 = -0.18428227254588e2_f64 * t12298 * t1020 - 0.18428227254588e2_f64 * t3749 * t2410 - 0.8704e0_f64 * t2956 * t3522 - 0.17408e1_f64 * t2410 * t3745 - 0.17408e1_f64 * t1020 * t12285 - 0.8704e0_f64 * t839 * t12894 - 0.9214113627294e1_f64 * t3524 * t2956 - 0.9214113627294e1_f64 * t3526 * t2956 - 0.9214113627294e1_f64 * t1129 * t9707 + 0.734774460522e2_f64 * t12300 * t1020 + 0.734774460522e2_f64 * t3753 * t2410 + 0.367387230261e2_f64 * t3530 * t2956 + 0.367387230261e2_f64 * t1131 * t9707 - 0.7662840944824e2_f64 * t12302 * t1020 - 0.7662840944824e2_f64 * t3757 * t2410;
    t44778
}
