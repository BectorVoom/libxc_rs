//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 591/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk591(t1165: f64, t4313: f64, t4314: f64, t1549: f64, t3382: f64, t1554: f64, t1558: f64, t1539: f64, t4289: f64, t1163: f64, t1173: f64, t1180: f64, t3396: f64, t4255: f64, t4258: f64, t4261: f64, t4264: f64, t4269: f64, t4275: f64, t4279: f64, t4280: f64, t4285: f64, t4288: f64, t4291: f64, t4295: f64, t4300: f64, t4304: f64, t4308: f64, t4310: f64, t4312: f64) -> (f64, f64, f64) {
    let t4316 = t1165 * t4313 * t4314;
    let t4320 = 0.85748036236139473944e-3_f64 * t3382 * t1549;
    let t4322 = 0.85748036236139473944e-3_f64 * t3382 * t1554;
    let t4324 = 0.42874018118069736972e-3_f64 * t3382 * t1558;
    let t4326 = t1165 * t4289 * t1539;
    let t4328 = 0.42874018118069736972e-3_f64 * t1163 * t4326;
    let t4329 = -t4255 * t4258 / 8.0_f64 - t4261 * t4264 / 12.0_f64 + 0.68598428988911579156e-2_f64 * t3396 * t4269 + t4275 - t4279 - 0.80031500487063509014e-2_f64 * t4280 - 0.85748036236139473944e-2_f64 * t4285 - t4288 + 0.17149607247227894789e-2_f64 * t1173 * t4291 - 0.85748036236139473944e-3_f64 * t1180 * t4295 + 0.85748036236139473944e-3_f64 * t1180 * t4300 - 0.42874018118069736972e-3_f64 * t1180 * t4304 + t4308 - t4310 + t4312 - 0.12862205435420921092e-2_f64 * t1180 * t4316 - t4320 + t4322 - t4324 + t4328;
    (t4316, t4326, t4329)
}
