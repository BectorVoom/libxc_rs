//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1005/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1005(t22704: f64, t22705: f64, t33280: f64, t33281: f64, t6914: f64, t1351: f64, t1992: f64, t550: f64, t6976: f64, t7918: f64, t1985: f64, t1998: f64, t214: f64, t27051: f64) -> (f64, f64, f64, f64) {
    let t122460 = t22704 * t22705 * t33280;
    let t122462 = t6914 * t33281;
    let t122467 = t1992 * t6976 * t7918 * t1351 * t550;
    let t122483 = t1985 * t214 * t1998 * t27051;
    (t122460, t122462, t122467, t122483)
}
