//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1340/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1340(t1799: f64, t2006: f64, t1307: f64, t26331: f64, t26446: f64, t1992: f64, t550: f64, t6976: f64, t90942: f64, t32745: f64, t6914: f64, t1351: f64, t7722: f64) -> (f64, f64, f64, f64, f64) {
    let t120437 = t2006 * t1799;
    let t120441 = 0.9869604401089358619e-1_f64 * t26331 * t26446 * t120437 * t1307;
    let t120445 = 0.16449340668482264365e-1_f64 * t1992 * t6976 * t90942 * t550;
    let t120446 = t6914 * t32745;
    let t120447 = 0.38381794893125283518e-1_f64 * t120446;
    let t120452 = 0.16449340668482264365e-1_f64 * t1992 * t6976 * t7722 * t1351 * t550;
    (t120437, t120441, t120445, t120447, t120452)
}
