//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 544/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk544(t227: f64, t297: f64, t4569: f64, t294: f64, t3293: f64, t565: f64, t806: f64, t564: f64, t1629: f64, t2053: f64, t1944: f64, sigma2: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t228 = t227 <= zeta_threshold;
    let t4570 = t297 * t4569;
    let t4571 = t294 * t4570;
    let t4573 = piecewise3(t228, 0.0_f64, t3293);
    let t4574 = t565 * t4573;
    let t4575 = t4574 * t806;
    let t4576 = t564 * t4575;
    let t4578 = t1629 * t2053;
    let t4579 = t564 * t4578;
    let t4581 = t1944 * sigma2;
    (t4570, t4571, t4574, t4576, t4579, t4581)
}
