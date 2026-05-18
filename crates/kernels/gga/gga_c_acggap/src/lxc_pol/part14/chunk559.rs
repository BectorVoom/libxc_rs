//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 559/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk559<F: Float>(t1549: F, t3382: F, t1554: F, t1558: F, t1165: F, t1539: F, t4289: F, t1163: F, t1490: F, t330: F, t3740: F, t527: F) -> (F, F, F, F, F, F, F) {
    let t4320 = F::new(0.85748036236139473944e-3) * t3382 * t1549;
    let t4322 = F::new(0.85748036236139473944e-3) * t3382 * t1554;
    let t4324 = F::new(0.42874018118069736972e-3) * t3382 * t1558;
    let t4326 = t1165 * t4289 * t1539;
    let t4328 = F::new(0.42874018118069736972e-3) * t1163 * t4326;
    let t4339 = F::new(7.0) / F::new(144.0) * t330 * t1490;
    let t4340 = t3740 * t527;
    (t4320, t4322, t4324, t4326, t4328, t4339, t4340)
}
