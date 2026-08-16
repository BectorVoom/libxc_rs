//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1260/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1260(t1841: f64, t3740: f64, t1016: f64, t1814: f64, t6102: f64, t997: f64, t1896: f64, t3670: f64, t1165: f64, t1180: f64, t1743: f64, t17821: f64, t17826: f64, t17831: f64, t17837: f64, t17851: f64, t17853: f64, t17855: f64, t398: f64, t4314: f64, t942: f64, t945: f64) -> (f64, f64) {
    let t23232 = t3740 * t1841;
    let t23234 = t1016 * t1814;
    let t23241 = t997 * t6102;
    let t23243 = t3670 * t1896;
    let t23250 = 0.85748036236139473944e-3_f64 * t942 * t398 * t1743 * t945 + 0.20007875121765877254e-2_f64 * t23232 - 0.12862205435420921092e-2_f64 * t1180 * t1165 * t23234 * t4314 - 0.80031500487063509016e-2_f64 * t17821 + 0.34299214494455789578e-1_f64 * t17826 + 0.40015750243531754508e-2_f64 * t23241 + 0.11337795902333997111e-1_f64 * t23243 + 0.40015750243531754508e-2_f64 * t17831 + 0.40015750243531754508e-1_f64 * t17837 - 35.0_f64 / 54.0_f64 * t17851 - 35.0_f64 / 54.0_f64 * t17853 - 0.80031500487063509016e-2_f64 * t17855;
    (t23234, t23250)
}
