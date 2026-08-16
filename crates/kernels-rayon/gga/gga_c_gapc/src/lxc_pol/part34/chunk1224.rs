//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1224/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1224(t11361: f64, t3060: f64, t9272: f64, t11604: f64, t26759: f64, t11326: f64, t27420: f64, t11308: f64, t11325: f64, t2993: f64, t20773: f64, t3712: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34495 = t3060 * t11361 * t9272;
    let t34497 = t11604 * t26759;
    let t34499 = t11326 * t27420;
    let t34501 = t11326 * t11308;
    let t34503 = t2993 * t11325;
    let t34505 = t34503 * t3712 * t20773;
    (t34495, t34497, t34499, t34501, t34503, t34505)
}
