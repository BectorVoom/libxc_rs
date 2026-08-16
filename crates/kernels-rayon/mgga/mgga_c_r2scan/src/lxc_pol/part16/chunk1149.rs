//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1149/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1149(t322: f64, t42547: f64, t1020: f64, t1083: f64, t1085: f64, t11979: f64, t11981: f64, t11983: f64, t11985: f64, t2410: f64, t2956: f64, t3388: f64, t3390: f64, t3394: f64, t3398: f64, t3650: f64, t3652: f64, t3656: f64, t3660: f64, t9707: f64) -> (f64, f64) {
    let t332 = 0.25e1_f64 < t322;
    let t42616 = piecewise3(t332, 0.0_f64, t42547);
    let t42646 = -0.64e0_f64 * t42616 - 0.18428227254588e2_f64 * t3650 * t2410 - 0.18428227254588e2_f64 * t11979 * t1020 - 0.18428227254588e2_f64 * t11981 * t1020 - 0.18428227254588e2_f64 * t3652 * t2410 - 0.9214113627294e1_f64 * t3388 * t2956 - 0.9214113627294e1_f64 * t3390 * t2956 - 0.9214113627294e1_f64 * t1083 * t9707 + 0.734774460522e2_f64 * t11983 * t1020 + 0.734774460522e2_f64 * t3656 * t2410 + 0.367387230261e2_f64 * t3394 * t2956 + 0.367387230261e2_f64 * t1085 * t9707 - 0.7662840944824e2_f64 * t11985 * t1020 - 0.7662840944824e2_f64 * t3660 * t2410 - 0.3831420472412e2_f64 * t3398 * t2956;
    (t42616, t42646)
}
