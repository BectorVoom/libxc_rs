//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 939/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk939(t140943: f64, t33434: f64, t33437: f64, t33446: f64, t33445: f64, t2336: f64, t7478: f64, t7477: f64, t172: f64, t33403: f64, t6056: f64, t27658: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t140945 = t33434 * t140943 * t33437;
    let t140959 = t140943 * t33446;
    let t140960 = t33445 * t140959;
    let t141002 = t7478 * t2336;
    let t141004 = 0.1304322025898084692e-2_f64 * t7477 * t141002;
    let t141051 = t33403 * t172;
    let t141052 = t141051 * t6056;
    let t141053 = t27658 * t141052;
    (t140945, t140959, t140960, t141002, t141004, t141051, t141052, t141053)
}
