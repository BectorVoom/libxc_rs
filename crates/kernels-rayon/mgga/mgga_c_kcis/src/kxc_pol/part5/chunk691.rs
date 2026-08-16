//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 691/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk691(t1122: f64, t5026: f64, t1092: f64, t1134: f64, t4999: f64, t1010: f64, t1710: f64, t300: f64, t3049: f64, t3247: f64, t3248: f64, t4768: f64, t4926: f64, t4978: f64, t4981: f64, t4987: f64, t4990: f64, t4997: f64, t5001: f64, t5003: f64, t5007: f64, t5011: f64, t5015: f64, t5017: f64, t5021: f64, t5023: f64, t979: f64) -> (f64, f64, f64, f64, f64) {
    let t5027 = t5026 * t1122;
    let t5028 = t1092 * t5027;
    let t5030 = t4999 * t1134;
    let t5031 = t1092 * t5030;
    let t5033 = 0.24872916666666666666e-2_f64 * t4926 - t3247 - 0.44218518518518518517e-2_f64 * t3248 - 0.66725e-1_f64 * t3049 * t1710 - 0.66725e-1_f64 * t979 * t4978 - 0.66725e-1_f64 * t4981 * t1010 - 0.16581944444444444444e-2_f64 * t4987 + 0.16581944444444444444e-2_f64 * t4990 + 0.33163888888888888888e-2_f64 * t4997 + 0.16581944444444444444e-2_f64 * t5001 + 0.11054629629629629629e-2_f64 * t5003 - 0.44218518518518518517e-2_f64 * t5007 + t4768 * t300 + 0.16581944444444444444e-2_f64 * t5011 - 0.44218518518518518517e-2_f64 * t5015 - 0.16581944444444444444e-2_f64 * t5017 + 0.66327777777777777776e-2_f64 * t5021 + 0.11054629629629629629e-2_f64 * t5023 - 0.24872916666666666666e-2_f64 * t5028 + 0.16581944444444444444e-2_f64 * t5031;
    (t5027, t5028, t5030, t5031, t5033)
}
