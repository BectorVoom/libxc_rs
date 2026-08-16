//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1060/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1060(t1181: f64, t34278: f64, t5641: f64, t599: f64, t5944: f64, t604: f64, t8463: f64, t5645: f64, t2001: f64, t5534: f64, t5559: f64, t1165: f64, t7351: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t38732 = t34278 * t1181 * t599 * t5641;
    let t38736 = t8463 * t1181 * t604 * t5944;
    let t38740 = t8463 * t1181 * t599 * t5645;
    let t38743 = t2001 * t5534;
    let t38747 = t8463 * t1181 * t604 * t5559;
    let t38751 = t8463 * t1165 * t7351 * t5944;
    (t38732, t38736, t38740, t38743, t38747, t38751)
}
