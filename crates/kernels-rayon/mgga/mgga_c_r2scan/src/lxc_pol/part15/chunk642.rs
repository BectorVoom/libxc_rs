//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 642/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk642(t3304: f64, t3594: f64, t2608: f64, t3308: f64, t574: f64, t1055: f64, t980: f64, t1060: f64, t938: f64) -> (f64, f64, f64, f64, f64) {
    let t3595 = t3304 * t3594;
    let t3597 = t3308 * t2608;
    let t3598 = t574 * t3597;
    let t3600 = t980 * t1055;
    let t3602 = t1060 * t938;
    (t3595, t3597, t3598, t3600, t3602)
}
