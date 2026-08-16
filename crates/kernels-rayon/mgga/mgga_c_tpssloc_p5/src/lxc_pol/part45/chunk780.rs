//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 780/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk780(t1926: f64, t3158: f64, t40: f64, t6722: f64, t1937: f64, t1929: f64, t34: f64, t1932: f64, t1934: f64, t6729: f64, t131: f64, t23322: f64) -> (f64, f64, f64, f64, f64) {
    let t23447 = t1926 * t3158 / 432.0_f64;
    let t23448 = t6722 * t40;
    let t23449 = t23448 * t1937;
    let t23451 = t1929 * t34;
    let t23452 = 1.0_f64 / t23451;
    let t23453 = t23452 * t1932;
    let t23454 = t23453 * t1934;
    let t23457 = t6722 * t6729;
    let t23460 = t23322 * t131;
    (t23447, t23449, t23454, t23457, t23460)
}
