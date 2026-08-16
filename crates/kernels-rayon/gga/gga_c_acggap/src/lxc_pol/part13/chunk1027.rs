//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1027/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1027(t34132: f64, t4680: f64, t7337: f64, t8902: f64, t2068: f64, t8907: f64, t1181: f64, t4540: f64, t604: f64, t7575: f64, t5111: f64, t4291: f64, t7561: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34133 = 0.37737710747524982482e-2_f64 * t34132;
    let t34135 = t7337 * t4680 * t8902;
    let t34138 = t2068 * t4680 * t8907;
    let t34142 = t7575 * t1181 * t604 * t4540;
    let t34146 = t7575 * t1181 * t604 * t5111;
    let t34148 = t7561 * t4291;
    (t34133, t34135, t34138, t34142, t34146, t34148)
}
