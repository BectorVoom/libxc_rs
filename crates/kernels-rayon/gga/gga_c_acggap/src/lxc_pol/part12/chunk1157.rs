//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1157/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1157(t33874: f64, t33886: f64, t33894: f64, t33903: f64, t30037: f64, t30061: f64, t30073: f64, t32339: f64, t32340: f64, t32341: f64, t32342: f64, t32348: f64, t32349: f64, t32350: f64, t33876: f64, t33881: f64, t33890: f64, t33898: f64) -> f64 {
    let t36833 = 0.10718504529517434243e-2_f64 * t33874;
    let t36836 = 0.57165357490759649296e-3_f64 * t33886;
    let t36838 = 0.28582678745379824648e-3_f64 * t33894;
    let t36841 = 0.57165357490759649296e-3_f64 * t33903;
    let t36842 = -0.13719685797782315831e-1_f64 * t30037 + t32339 + t32340 + t32341 + t32342 + 0.42874018118069736972e-2_f64 * t30061 - t36833 - 0.18007087609589289528e-1_f64 * t33876 - 0.42874018118069736972e-3_f64 * t33881 - t36836 - 0.85748036236139473944e-3_f64 * t33890 - t36838 - 0.85748036236139473944e-3_f64 * t33898 - 0.17149607247227894789e-2_f64 * t30073 - t36841 - t32348 - t32349 + t32350;
    t36842
}
