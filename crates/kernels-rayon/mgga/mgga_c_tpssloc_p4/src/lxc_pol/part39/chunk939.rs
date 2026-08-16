//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 939/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk939(t2374: f64, t9888: f64, t2509: f64, t745: f64, t9843: f64, t761: f64, t152: f64, t31: f64, t2448: f64, t67: f64, t758: f64, t2368: f64, t2505: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9890 = 0.48159733137676571078e0_f64 * t2374 * t9888;
    let t9892 = t2509 * t745 * t9843;
    let t9894 = 0.51947577317044391277e2_f64 * t761 * t9892;
    let t9897 = t31 * t152;
    let t9901 = t2448 * t67;
    let t9902 = t9901 * t758;
    let t9905 = t2368 * t745 * t2505;
    (t9890, t9892, t9894, t9897, t9902, t9905)
}
