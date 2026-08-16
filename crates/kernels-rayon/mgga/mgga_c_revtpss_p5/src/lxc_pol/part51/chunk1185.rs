//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1185/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1185(t33976: f64, t7235: f64, t119578: f64, t28067: f64, t28167: f64, t37972: f64, t5627: f64, t28177: f64, t8568: f64, t34258: f64, t7003: f64, t2014: f64, t49575: f64, t8599: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t127335 = t7235 * t33976;
    let t127336 = t119578 * t28067;
    let t127340 = 6.0_f64 * t28167 * t37972 * t5627;
    let t127341 = t8568 * t28177;
    let t127346 = 4.0_f64 * t34258 * t7003;
    let t127349 = 2.0_f64 * t2014 * t8599 * t49575;
    (t127335, t127336, t127340, t127341, t127346, t127349)
}
