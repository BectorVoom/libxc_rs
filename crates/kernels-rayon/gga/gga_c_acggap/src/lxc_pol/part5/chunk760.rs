//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 760/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk760(t406: f64, t5752: f64, t1532: f64, t1181: f64, t1881: f64, t997: f64, t1173: f64, t1531: f64, t3209: f64, t3215: f64, t3218: f64, t3229: f64, t3231: f64, t3233: f64, t3238: f64, t3240: f64, t3403: f64, t3462: f64, t4459: f64, t4462: f64, t5712: f64, t5717: f64, t5722: f64, t5728: f64, t5733: f64, t5737: f64, t5743: f64, t5749: f64) -> (f64, f64, f64, f64) {
    let t5753 = t5752 * t406;
    let t5754 = t1532 * t5753;
    let t5755 = t1181 * t5754;
    let t5758 = t997 * t1881;
    let t5766 = 0.34299214494455789578e-2_f64 * t1173 * t5712 - 0.34299214494455789578e-2_f64 * t1173 * t5717 + 0.17149607247227894789e-2_f64 * t1173 * t5722 + 0.85748036236139473944e-3_f64 * t5728 - 0.17149607247227894789e-2_f64 * t5733 - 0.85748036236139473945e-2_f64 * t3403 * t5737 - 0.17149607247227894789e-2_f64 * t1531 * t5743 - 0.34299214494455789578e-2_f64 * t3462 * t5749 + 0.17149607247227894789e-2_f64 * t1531 * t5755 - t3209 - 0.60023625365297631763e-2_f64 * t5758 + 0.85748036236139473944e-3_f64 * t4459 + t4462 - t3215 - t3218 - 0.85748036236139473944e-3_f64 * t3229 + 0.42874018118069736972e-3_f64 * t3231 - 0.42874018118069736972e-3_f64 * t3233 - 0.40015750243531754508e-2_f64 * t3238 + 0.40015750243531754508e-2_f64 * t3240;
    (t5753, t5754, t5755, t5766)
}
