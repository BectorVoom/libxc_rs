//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1086/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1086(t2937: f64, t524: f64, t943: f64, t1165: f64, t30856: f64, t604: f64, t33751: f64, t7413: f64, t1181: f64, t599: f64, t2297: f64, t3176: f64) -> (f64, f64, f64, f64, f64) {
    let t35324 = t524 * t2937 * t943;
    let t35327 = t30856 * t1165 * t604 * t35324;
    let t35331 = t7413 * t1165 * t604 * t33751;
    let t35335 = t30856 * t1181 * t599 * t35324;
    let t35340 = t2297 * t3176;
    (t35324, t35327, t35331, t35335, t35340)
}
