//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 816/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk816(t1985: f64, t26351: f64, t1842: f64, t3886: f64, t1385: f64, t22635: f64, t1992: f64, t6883: f64, t7697: f64, t22897: f64, t5336: f64, t22751: f64, t7733: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26352 = t1985 * t26351;
    let t26354 = t3886 * t1842;
    let t26355 = t26354 * t1385;
    let t26356 = t22635 * t26355;
    let t26357 = t1992 * t26356;
    let t26361 = t6883 * t7697;
    let t26378 = t22897 * t5336;
    let t26379 = t1992 * t26378;
    let t26381 = t22751 * t7733;
    (t26352, t26355, t26357, t26361, t26379, t26381)
}
