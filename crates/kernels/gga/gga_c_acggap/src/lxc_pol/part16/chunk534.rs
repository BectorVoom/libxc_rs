//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 534/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk534<F: Float>(t1410: F, t435: F, t1549: F, t3409: F, t1554: F, t1558: F, t1016: F, t524: F, t3382: F, t1165: F, t1539: F, t4289: F, t1163: F, t1490: F, t330: F, t3740: F, t527: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t4298 = t435 * t1410;
    let t4308 = 0.40015750243531754508e-2 * t3409 * t1549;
    let t4310 = 0.40015750243531754508e-2 * t3409 * t1554;
    let t4312 = 0.20007875121765877254e-2 * t3409 * t1558;
    let t4313 = t1016 * t524;
    let t4320 = 0.85748036236139473944e-3 * t3382 * t1549;
    let t4322 = 0.85748036236139473944e-3 * t3382 * t1554;
    let t4324 = 0.42874018118069736972e-3 * t3382 * t1558;
    let t4326 = t1165 * t4289 * t1539;
    let t4328 = 0.42874018118069736972e-3 * t1163 * t4326;
    let t4339 = 7.0 / 144.0 * t330 * t1490;
    let t4340 = t3740 * t527;
    (t4298, t4308, t4310, t4312, t4313, t4320, t4322, t4324, t4326, t4328, t4339, t4340)
}
