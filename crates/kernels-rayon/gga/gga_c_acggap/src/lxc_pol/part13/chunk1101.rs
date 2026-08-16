//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1101/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1101(t35096: f64, t1181: f64, t21118: f64, t7351: f64, t7426: f64, t1131: f64, t525: f64, t2068: f64, t604: f64, t33706: f64, t599: f64, t1165: f64, t21955: f64, t30806: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35097 = 0.21437009059034868486e-2_f64 * t35096;
    let t35100 = t7426 * t1181 * t7351 * t21118;
    let t35101 = 0.12862205435420921092e-2_f64 * t35100;
    let t35102 = t525 * t1131;
    let t35105 = t2068 * t1181 * t604 * t35102;
    let t35109 = t2068 * t1181 * t599 * t33706;
    let t35113 = t30806 * t1165 * t604 * t21955;
    (t35097, t35101, t35102, t35105, t35109, t35113)
}
