//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1061/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1061(t101: f64, t1501: f64, t159: f64, t16580: f64, t19087: f64, t19107: f64, t19108: f64, t19117: f64, t19121: f64, t19124: f64, t19129: f64, t19132: f64, t19136: f64, t19138: f64, t19140: f64, t19143: f64, t19148: f64, t19152: f64, t19157: f64, t19161: f64, t19165: f64, t19169: f64, t2033: f64, t2035: f64, t2037: f64, t281: f64, t285: f64, t523: f64, t5603: f64, t5881: f64, t8331: f64) -> f64 {
    let t19170 = t19107 + 12.0_f64 * t2035 * t19108 - 0.11974234010254609094e-1_f64 * t281 * t16580 * t159 * t285 - 0.47896936041018436376e-1_f64 * t19117 - t19121 - 3.0_f64 * t5881 * t2033 + 12.0_f64 * t19124 * t2037 + 6.0_f64 * t101 * t19087 * t19129 + 36.0_f64 * t19132 * t5603 + t523 * t19136 - 36.0_f64 * t19138 * t19140 + 72.0_f64 * t8331 * t19143 - 0.71845404061527654564e-1_f64 * t19148 - 0.47896936041018436376e-1_f64 * t19152 - t19157 + 3.0_f64 * t5881 * t1501 + 6.0_f64 * t523 * t19161 - 0.26861343269868796571e-1_f64 * t19165 - t19169;
    t19170
}
