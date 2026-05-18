//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 591/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk591<F: Float>(t1165: F, t4313: F, t4314: F, t1549: F, t3382: F, t1554: F, t1558: F, t1539: F, t4289: F, t1163: F, t1173: F, t1180: F, t3396: F, t4255: F, t4258: F, t4261: F, t4264: F, t4269: F, t4275: F, t4279: F, t4280: F, t4285: F, t4288: F, t4291: F, t4295: F, t4300: F, t4304: F, t4308: F, t4310: F, t4312: F) -> (F, F, F) {
    let t4316 = t1165 * t4313 * t4314;
    let t4320 = F::new(0.85748036236139473944e-3) * t3382 * t1549;
    let t4322 = F::new(0.85748036236139473944e-3) * t3382 * t1554;
    let t4324 = F::new(0.42874018118069736972e-3) * t3382 * t1558;
    let t4326 = t1165 * t4289 * t1539;
    let t4328 = F::new(0.42874018118069736972e-3) * t1163 * t4326;
    let t4329 = -t4255 * t4258 / F::new(8.0) - t4261 * t4264 / F::new(12.0) + F::new(0.68598428988911579156e-2) * t3396 * t4269 + t4275 - t4279 - F::new(0.80031500487063509014e-2) * t4280 - F::new(0.85748036236139473944e-2) * t4285 - t4288 + F::new(0.17149607247227894789e-2) * t1173 * t4291 - F::new(0.85748036236139473944e-3) * t1180 * t4295 + F::new(0.85748036236139473944e-3) * t1180 * t4300 - F::new(0.42874018118069736972e-3) * t1180 * t4304 + t4308 - t4310 + t4312 - F::new(0.12862205435420921092e-2) * t1180 * t4316 - t4320 + t4322 - t4324 + t4328;
    (t4316, t4326, t4329)
}
