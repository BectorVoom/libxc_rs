//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1171/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1171(t3431: f64, t5722: f64, t1165: f64, t3194: f64, t5284: f64, t5852: f64, t1173: f64, t1181: f64, t13031: f64, t13040: f64, t13065: f64, t1531: f64, t16328: f64, t16332: f64, t16356: f64, t16359: f64, t16373: f64, t1899: f64, t3196: f64, t5136: f64, t5862: f64) -> f64 {
    let t21166 = t3431 * t5722;
    let t21170 = t3194 * t1165 * t5852 * t5284;
    let t21182 = -0.13719685797782315831e-1_f64 * t16328 + 0.20579528696673473746e-1_f64 * t16332 - 0.34299214494455789578e-2_f64 * t1173 * t1165 * t1899 * t3196 - 0.16006300097412701803e-1_f64 * t21166 - 0.17149607247227894789e-2_f64 * t21170 - 0.34299214494455789578e-2_f64 * t13031 - 0.42874018118069736972e-3_f64 * t13040 - 0.85748036236139473944e-3_f64 * t16356 + 455.0_f64 / 324.0_f64 * t13065 + 35.0_f64 / 108.0_f64 * t16359 - 0.51448821741683684366e-2_f64 * t16373 + 0.17149607247227894789e-2_f64 * t1531 * t1181 * t5862 * t5136;
    t21182
}
