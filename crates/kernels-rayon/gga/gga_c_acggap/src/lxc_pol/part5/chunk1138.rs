//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1138/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1138(t3382: f64, t5986: f64, t3409: f64, t5801: f64, t12738: f64, t6184: f64, t1165: f64, t1173: f64, t12498: f64, t12511: f64, t12516: f64, t12529: f64, t12532: f64, t1889: f64, t20400: f64, t3176: f64, t3196: f64, t3403: f64, t4289: f64, t5720: f64, t5862: f64) -> f64 {
    let t20405 = t3382 * t5986;
    let t20407 = t3409 * t5801;
    let t20409 = t12738 * t6184;
    let t20416 = -0.85748036236139473944e-2_f64 * t3403 * t1165 * t1889 * t3196 + 0.17149607247227894789e-2_f64 * t1173 * t1165 * t5862 * t3196 + 0.34299214494455789578e-2_f64 * t1173 * t1165 * t4289 * t5720 - 0.34299214494455789578e-2_f64 * t1173 * t1165 * t20400 * t3176 - 0.51448821741683684366e-2_f64 * t20405 + 0.80031500487063509014e-2_f64 * t20407 + 0.40015750243531754508e-2_f64 * t20409 - 0.24009450146119052705e-1_f64 * t12498 + 0.24009450146119052705e-1_f64 * t12511 + 0.85748036236139473944e-3_f64 * t12516 + 0.85748036236139473944e-3_f64 * t12529 - 0.34013387707001991332e-1_f64 * t12532;
    t20416
}
