//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1206/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1206(t2299: f64, t3689: f64, t1415: f64, t1646: f64, t1: f64, t544: f64, t594: f64, t2392: f64, t47953: f64, t6710: f64, t6711: f64, t12092: f64, t2478: f64, t6583: f64) -> (f64, f64, f64, f64) {
    let t48165 = t2299 * t3689;
    let t48167 = t1415 * t48165 * t1646;
    let t48171 = t544 * t594 * t3689 * t1;
    let t48172 = t48171 * t2392;
    let t48175 = t6710 * t6711 * t47953;
    let t48178 = t6583 * t12092 * t2478;
    (t48167, t48172, t48175, t48178)
}
