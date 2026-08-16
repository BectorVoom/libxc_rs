//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 884/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk884(t2148: f64, t8066: f64, t2147: f64, t113: f64, t7503: f64, t2115: f64, t2155: f64, t6063: f64, t7619: f64, t537: f64, t7624: f64, t560: f64, t921: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8067 = t2148 * t8066;
    let t8069 = 0.11643651550782197811e-1_f64 * t2147 * t8067;
    let t8070 = t7503 * t113;
    let t8071 = t2115 * t8070;
    let t8073 = 0.97574405393827830186e-2_f64 * t2155 * t8071;
    let t8074 = t6063 * t7619;
    let t8076 = 0.19514881078765566037e-1_f64 * t2155 * t8074;
    let t8077 = t2115 * t537;
    let t8078 = t8077 * t7624;
    let t8080 = 0.97574405393827830186e-2_f64 * t2155 * t8078;
    let t8081 = t921 * t560;
    (t8069, t8070, t8071, t8073, t8076, t8080, t8081)
}
