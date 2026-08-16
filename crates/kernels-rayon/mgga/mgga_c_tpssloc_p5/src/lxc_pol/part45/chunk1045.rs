//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1045/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1045(t5: f64, t115861: f64, t115911: f64, t112: f64, t114387: f64, t114388: f64, t114405: f64, t114413: f64, t114415: f64, t115813: f64, t115815: f64, t115817: f64, t115819: f64, t115821: f64, t115824: f64, t2039: f64, t26103: f64, t7056: f64, t83935: f64) -> (f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t115913 = piecewise3(t8, 0.0_f64, t115861 + t115911);
    let t115914 = t115913 * t112;
    let t115915 = 2.0_f64 * t2039 * t83935 + 4.0_f64 * t26103 * t7056 + t114387 + t114388 + t114405 + t114413 + t114415 + t115813 + t115815 + t115817 + t115819 + t115821 + 2.0_f64 * t115824 + t115914;
    (t115914, t115915)
}
