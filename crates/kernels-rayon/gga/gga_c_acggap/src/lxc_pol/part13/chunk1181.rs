//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1181/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1181(t30120: f64, t8948: f64, t4680: f64, t7413: f64, t8947: f64, t1181: f64, t2068: f64, t20972: f64, t604: f64, t21128: f64, t7839: f64, t8787: f64) -> (f64, f64, f64, f64, f64) {
    let t36162 = t30120 * t8948;
    let t36163 = 0.42874018118069736972e-3_f64 * t36162;
    let t36165 = t7413 * t4680 * t8947;
    let t36169 = t2068 * t1181 * t604 * t20972;
    let t36173 = t2068 * t1181 * t604 * t21128;
    let t36175 = t7839 * t8787;
    (t36163, t36165, t36169, t36173, t36175)
}
