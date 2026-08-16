//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1062/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1062(t3382: f64, t4443: f64, t12746: f64, t1530: f64, t1535: f64, t12743: f64, t1562: f64, t3431: f64, t4410: f64, t14056: f64, t4269: f64, t1111: f64, t13299: f64, t17173: f64, t406: f64, t8790: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18765 = t3382 * t4443;
    let t18768 = t1530 * t12746 * t1535;
    let t18770 = t12743 * t1562;
    let t18772 = t3431 * t4410;
    let t18788 = t14056 * t4269;
    let t18805 = t17173 * t13299 * t8790 * t1111 * t406;
    (t18765, t18768, t18770, t18772, t18788, t18805)
}
