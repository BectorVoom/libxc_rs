//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 850/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk850(t1165: f64, t1799: f64, t2056: f64, t4347: f64, t5799: f64, t5801: f64, t5815: f64, t645: f64, t1844: f64, t508: f64) -> (f64, f64) {
    let t5905 = 2.0_f64 * t1165 * t5815 + 2.0_f64 * t1799 * t2056 + 2.0_f64 * t1799 * t4347 + 2.0_f64 * t5801 * t645 + t5799;
    let t5909 = t508 * t1844;
    (t5905, t5909)
}
