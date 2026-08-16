//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 921/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk921(t12328: f64, t555: f64, t10027: f64, t541: f64, t3777: f64, t3865: f64, t1361: f64, t2690: f64, t1336: f64, t1369: f64, t241: f64, t67: f64, t6924: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12330 = 595.0_f64 / 10368.0_f64 * t555 * t12328;
    let t12335 = 455.0_f64 / 1296.0_f64 * t10027 * t541;
    let t12339 = t3777 * t3865;
    let t12344 = t1361 * t2690;
    let t12345 = t1336 * t12344;
    let t12346 = t12345 * t1369;
    let t12351 = t241 * t6924 * t67;
    (t12330, t12335, t12339, t12345, t12346, t12351)
}
