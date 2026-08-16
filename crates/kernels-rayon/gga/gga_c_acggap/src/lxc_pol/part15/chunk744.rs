//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 744/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk744(t2217: f64, t315: f64, t323: f64, t2176: f64, t872: f64, t7311: f64, t7327: f64, t7372: f64, t7375: f64, t7378: f64, t7462: f64, t7515: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8114 = t315 * t2217;
    let t8115 = t8114 * t323;
    let t8123 = t2176 * t872;
    let t8129 = 0.1324375e0_f64 * t7311;
    let t8133 = 0.7640625e-2_f64 * t7327;
    let t8144 = 0.22675591804667994221e-1_f64 * t7372;
    let t8145 = 0.80031500487063509014e-2_f64 * t7375;
    let t8146 = 0.85748036236139473944e-3_f64 * t7378;
    let t8171 = 0.28582678745379824648e-3_f64 * t7462;
    let t8184 = 0.85748036236139473944e-3_f64 * t7515;
    (t8114, t8115, t8123, t8129, t8133, t8144, t8145, t8146, t8171, t8184)
}
