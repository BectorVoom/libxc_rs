//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1329/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1329(t72: f64, t79: f64, t9342: f64, t531: f64, t6995: f64, t1983: f64, t22596: f64, t12012: f64, t1390: f64, t6878: f64, t22574: f64, t39367: f64, t8643: f64) -> (f64, f64, f64, f64) {
    let t83846 = t72 * t79 * t9342;
    let t83859 = t531 * t6995;
    let t83862 = 18.0_f64 * t1983 * t83859 * t22596;
    let t83863 = t1390 * t12012;
    let t83866 = 3.0_f64 * t1983 * t6878 * t83863;
    let t83869 = 9.0_f64 * t22574 * t8643 * t39367;
    (t83846, t83862, t83866, t83869)
}
