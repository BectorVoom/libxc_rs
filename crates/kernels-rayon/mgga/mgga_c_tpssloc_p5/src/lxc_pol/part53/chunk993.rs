//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 993/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk993(t7841: f64, t857: f64, t22986: f64, t23270: f64, t776: f64, t31338: f64, t86873: f64, t33422: f64, t6547: f64, t114601: f64, t1527: f64, t1888: f64) -> (f64, f64, f64, f64) {
    let t121634 = t857 * t7841;
    let t121637 = t22986 * t23270 * t121634 * t776;
    let t121648 = t22986 * t86873 * t31338;
    let t121660 = t6547 * t33422;
    let t121689 = t1888 * t23270 * t114601 * t1527;
    (t121637, t121648, t121660, t121689)
}
