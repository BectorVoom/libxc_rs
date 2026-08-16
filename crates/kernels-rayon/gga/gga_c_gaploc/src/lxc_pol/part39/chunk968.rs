//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 968/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk968(t10171: f64, t2317: f64, t6525: f64, t2321: f64, t34478: f64, t9074: f64, t123: f64, t31730: f64, t2326: f64, t12797: f64, t1358: f64, t12773: f64, t6305: f64) -> (f64, f64, f64, f64, f64) {
    let t42661 = t6525 * t10171 * t2317;
    let t42664 = t9074 * t34478 * t2321;
    let t42669 = t31730 * t123;
    let t42671 = t9074 * t42669 * t2326;
    let t42673 = t1358 * t12797;
    let t42674 = 0.31616674039640166221e-2_f64 * t42673;
    let t42675 = t6305 * t12773;
    (t42661, t42664, t42671, t42674, t42675)
}
