//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 779/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk779(t22634: f64, t2684: f64, t9438: f64, t22629: f64, t825: f64, t900: f64, t9624: f64, t12960: f64, t1537: f64, t34890: f64, t6583: f64, t9537: f64) -> (f64, f64, f64, f64, f64) {
    let t41448 = t2684 * t9438 * t22634;
    let t41477 = t825 * t9438 * t22629;
    let t41511 = t900 * t9624;
    let t41594 = t1537 * t12960;
    let t41606 = t6583 * t34890 * t9537;
    (t41448, t41477, t41511, t41594, t41606)
}
