//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1004/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1004(t3585: f64, t820: f64, t1216: f64, t3243: f64, t1090: f64, t3494: f64, t3578: f64, t10401: f64, t3575: f64, t3610: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11668 = t820 * t3585;
    let t11669 = t1216 * t3243;
    let t11670 = t11668 * t11669;
    let t11673 = t3494 * t1090;
    let t11674 = t3578 * t11673;
    let t11677 = t3575 * t10401;
    let t11678 = t3610 * t11677;
    (t11668, t11669, t11670, t11673, t11674, t11677, t11678)
}
