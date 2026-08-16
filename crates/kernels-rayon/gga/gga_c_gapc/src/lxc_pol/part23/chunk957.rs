//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 957/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk957(t125: f64, t818: f64, t329: f64, t2536: f64, t1062: f64, t268: f64, t3643: f64, t128: f64, t6939: f64, t10357: f64, t2207: f64, t10350: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11668 = t125 * t818;
    let t11669 = t11668 * t329;
    let t11670 = t11669 * t2536;
    let t11671 = t1062 * t11670;
    let t11673 = t3643 * t268;
    let t11674 = t6939 * t128;
    let t11675 = t11673 * t11674;
    let t11676 = t11675 * t10357;
    let t11678 = t2207 * t128;
    let t11679 = t11673 * t11678;
    let t11680 = t11679 * t10350;
    (t11669, t11670, t11671, t11673, t11674, t11675, t11676, t11678, t11679, t11680)
}
