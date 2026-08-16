//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1025/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1025(t171: f64, t2426: f64, t3771: f64, t6793: f64, t6789: f64, t79931: f64, t679: f64, t123129: f64, t123133: f64, t141004: f64, t141107: f64, t150547: f64, t150552: f64, t150554: f64, t150558: f64, t17807: f64, t27557: f64, t27561: f64, t27629: f64, t27717: f64, t33359: f64, t33394: f64, t33445: f64, t35437: f64, t6057: f64, t689: f64, t690: f64, t7853: f64) -> f64 {
    let t150565 = t3771 * t2426 * t6793 * t171;
    let t150569 = t79931 * t6789;
    let t150570 = t6793 * t679;
    let t150577 = 0.3827206426927081041e-8_f64 * t17807 * t141107 * t27561 + 0.45958162518691859408e-7_f64 * t17807 * t33394 * t27557 - 0.20869152414369355073e-1_f64 * t33445 * t150547 - 0.60548059007656442387e-3_f64 * t150552 - 0.25537443351851851852e-1_f64 * t150554 * t6057 - 0.45497819271775541929e-4_f64 * t150558 + 0.88910709717637694816e-2_f64 * t27717 * t7853 * t27629 - t141004 - 0.49184261954149446141e-6_f64 * t150565 * t35437 * t690 + 0.24511020009968991683e-5_f64 * t123133 * t150569 * t150570 * t689 + 0.10338048737805743097e-3_f64 * t123129 * t33359;
    t150577
}
