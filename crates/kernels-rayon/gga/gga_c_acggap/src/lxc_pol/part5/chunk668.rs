//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 668/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk668(t1089: f64, t175: f64, t4555: f64, t384: f64, t1429: f64, t997: f64, t1418: f64, t1347: f64, t1165: f64, t1532: f64, t3084: f64, t1531: f64, t3306: f64, t3308: f64, t3310: f64, t3312: f64, t3314: f64, t3316: f64, t418: f64, t4518: f64, t4524: f64, t4528: f64, t4532: f64, t4535: f64, t4538: f64, t4542: f64, t4547: f64, t4552: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4557 = t1089 * t175 * t4555;
    let t4558 = t384 * t4557;
    let t4561 = 0.40015750243531754508e-1_f64 * t997 * t1429;
    let t4563 = 0.16006300097412701803e-1_f64 * t997 * t1418;
    let t4565 = 0.16006300097412701803e-1_f64 * t997 * t1347;
    let t4567 = t1165 * t1532 * t3084;
    let t4570 = -0.17149607247227894789e-2_f64 * t3306 - 0.34299214494455789578e-2_f64 * t3308 + 0.34299214494455789578e-2_f64 * t3310 + 0.34299214494455789577e-2_f64 * t3312 - 0.17149607247227894789e-2_f64 * t3314 + 0.17149607247227894789e-2_f64 * t3316 - 0.85748036236139473944e-3_f64 * t418 * t4518 + 0.42874018118069736972e-3_f64 * t4524 - 0.85748036236139473944e-3_f64 * t418 * t4528 - t4532 + 0.17149607247227894789e-2_f64 * t418 * t4535 + 0.80031500487063509015e-2_f64 * t4538 - 0.17149607247227894789e-2_f64 * t418 * t4542 - 0.17149607247227894789e-2_f64 * t418 * t4547 + 0.17149607247227894789e-2_f64 * t418 * t4552 + 0.85748036236139473944e-3_f64 * t4558 - t4561 + t4563 - t4565 + 0.42874018118069736972e-3_f64 * t1531 * t4567;
    (t4557, t4558, t4561, t4563, t4565, t4567, t4570)
}
