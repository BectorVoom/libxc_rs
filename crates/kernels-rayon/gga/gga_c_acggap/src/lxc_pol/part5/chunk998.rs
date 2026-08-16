//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 998/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk998(t1008: f64, t4886: f64, t14106: f64, t532: f64, t1569: f64, t3670: f64, t3216: f64, t5101: f64, t3382: f64, t4414: f64, t1101: f64, t1181: f64, t1579: f64, t3361: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16694 = t1008 * t4886;
    let t16701 = t14106 * t532;
    let t16703 = t3670 * t1569;
    let t16705 = t3216 * t5101;
    let t16707 = t3382 * t4414;
    let t16720 = t3361 * t1181 * t1579 * t1101;
    (t16694, t16701, t16703, t16705, t16707, t16720)
}
