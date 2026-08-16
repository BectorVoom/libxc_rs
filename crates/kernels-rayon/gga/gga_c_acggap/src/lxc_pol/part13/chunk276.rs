//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 276/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk276(t1036: f64, t1039: f64, t1000: f64, t1002: f64, t1007: f64, t1009: f64, t1011: f64, t1013: f64, t1020: f64, t1029: f64, t1034: f64, t418: f64, t995: f64, t998: f64) -> (f64, f64) {
    let t1041 = 0.42874018118069736972e-3_f64 * t1036 * t1039;
    let t1042 = -t995 + 0.80031500487063509015e-2_f64 * t998 - 0.40015750243531754508e-2_f64 * t1000 + 0.40015750243531754508e-2_f64 * t1002 - t1007 - 0.17149607247227894789e-2_f64 * t1009 + 0.85748036236139473944e-3_f64 * t1011 - 0.85748036236139473944e-3_f64 * t1013 + 0.12862205435420921092e-2_f64 * t418 * t1020 + 0.42874018118069736972e-2_f64 * t418 * t1029 + t1034 + t1041;
    (t1041, t1042)
}
