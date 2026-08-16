//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 876/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk876(t31623: f64, t6897: f64, t1351: f64, t2085: f64, t550: f64, t6976: f64, t1992: f64, t1998: f64, t7191: f64, t214: f64, t1985: f64, t31611: f64, t6891: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t31624 = t6897 * t31623;
    let t31627 = t2085 * t1351 * t550;
    let t31628 = t6976 * t31627;
    let t31629 = t1992 * t31628;
    let t31631 = t1998 * t7191;
    let t31632 = t214 * t31631;
    let t31633 = t1985 * t31632;
    let t31645 = t31611 * t6891;
    (t31624, t31627, t31628, t31629, t31631, t31632, t31633, t31645)
}
