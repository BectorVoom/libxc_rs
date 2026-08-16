//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1132/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1132(t3295: f64, t4039: f64, t4022: f64, t863: f64, t6523: f64, t8867: f64, t1150: f64, t14028: f64, t14046: f64, t4171: f64, t3268: f64, t4049: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14544 = t4039 * t3295;
    let t14547 = t863 * t4022;
    let t14548 = t6523 * t8867;
    let t14549 = t14547 * t14548;
    let t14551 = t14028 * t1150;
    let t14554 = t14046 * t4171;
    let t14556 = t4049 * t3268;
    (t14544, t14547, t14548, t14549, t14551, t14554, t14556)
}
