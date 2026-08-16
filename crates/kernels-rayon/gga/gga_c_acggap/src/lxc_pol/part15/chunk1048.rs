//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1048/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1048(t34390: f64, t34398: f64, t34409: f64, t34421: f64, t34429: f64, t34488: f64, t34500: f64, t34506: f64, t34512: f64, t34534: f64, t34537: f64, t34556: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t37065 = 0.1120625e0_f64 * t34390;
    let t37069 = 0.11321313224257494745e-1_f64 * t34398;
    let t37076 = 0.42874018118069736972e-3_f64 * t34409;
    let t37087 = 7.0_f64 / 72.0_f64 * t34421;
    let t37090 = 0.21437009059034868486e-2_f64 * t34429;
    let t37121 = 0.916875e-1_f64 * t34488;
    let t37126 = 0.68598428988911579156e-2_f64 * t34500;
    let t37129 = 0.34299214494455789578e-2_f64 * t34506;
    let t37132 = 0.32012600194825403606e-1_f64 * t34512;
    let t37140 = 0.34299214494455789578e-2_f64 * t34534;
    let t37142 = 0.17149607247227894789e-2_f64 * t34537;
    let t37150 = 0.12579236915841660828e-2_f64 * t34556;
    (t37065, t37069, t37076, t37087, t37090, t37121, t37126, t37129, t37132, t37140, t37142, t37150)
}
