//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1278/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1278(t384: f64, t398: f64, t429: f64, t6192: f64, t1089: f64, t1193: f64, t14113: f64, t14115: f64, t14117: f64, t14120: f64, t14122: f64, t18295: f64, t18297: f64, t18299: f64, t18301: f64, t418: f64, t422: f64, t5679: f64, t5876: f64) -> f64 {
    let t23636 = t384 * t398 * t429 * t6192;
    let t23650 = -0.85748036236139473944e-3_f64 * t14113 + 0.42874018118069736972e-3_f64 * t14115 - 0.42874018118069736972e-3_f64 * t14117 + t14120 + t14122 - 0.85748036236139473944e-3_f64 * t23636 - 0.34299214494455789578e-2_f64 * t418 * t1089 * t429 * t5876 - 0.17149607247227894789e-2_f64 * t418 * t422 * t5679 * t1193 + 0.16006300097412701803e-1_f64 * t18295 - 0.16006300097412701803e-1_f64 * t18297 + 0.80031500487063509016e-2_f64 * t18299 + 0.34299214494455789578e-2_f64 * t18301;
    t23650
}
