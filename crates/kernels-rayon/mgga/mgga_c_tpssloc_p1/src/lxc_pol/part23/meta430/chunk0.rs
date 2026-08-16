//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1266/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1266(t1098: f64, t21988: f64, t21938: f64, t3400: f64, t19080: f64, t4997: f64, t19047: f64, t19040: f64, t5005: f64, t19026: f64, t18975: f64, t11719: f64, t22307: f64, t248: f64, t3570: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t71877 = t21988 * t1098;
    let t72062 = t3400 * t21938;
    let t72161 = t19080 * t4997;
    let t72181 = t19047 * t4997;
    let t72183 = t5005 * t19040;
    let t72223 = t19026 * t4997;
    let t72225 = t5005 * t18975;
    let t72229 = t11719 * t248 * t3570 * t22307;
    (t71877, t72062, t72161, t72181, t72183, t72223, t72225, t72229)
}
