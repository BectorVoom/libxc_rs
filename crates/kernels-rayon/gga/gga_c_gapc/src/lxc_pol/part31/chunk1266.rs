//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1266/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1266(t11379: f64, t11381: f64, t25953: f64, t28065: f64, t3709: f64, t11388: f64, t8636: f64, t11380: f64, t1448: f64, t8788: f64, t11509: f64, t5626: f64) -> (f64, f64, f64, f64, f64) {
    let t34980 = t25953 * t11379 * t11381;
    let t34982 = t3709 * t28065;
    let t34984 = t11388 * t8636;
    let t34987 = t11380 * t1448 * t8788;
    let t34989 = t11509 * t5626;
    (t34980, t34982, t34984, t34987, t34989)
}
