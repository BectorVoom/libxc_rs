//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1088/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1088(t1699: f64, t3225: f64, t2628: f64, t7340: f64, t22537: f64, t822: f64, t20671: f64, t22634: f64, t2012: f64, t9804: f64, t9807: f64, t21446: f64, t5641: f64, t883: f64, t9805: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t27860 = t3225 * t1699;
    let t28022 = 0.11916829983950142223e0_f64 * t7340 * t2628;
    let t28069 = t822 * t22537;
    let t28072 = 0.85206502119823888169e0_f64 * t28069 * t20671 * t22634;
    let t28073 = t2012 * t9804;
    let t28075 = 0.23005755572352449806e1_f64 * t28073 * t9807;
    let t28079 = 0.23005755572352449806e1_f64 * t9805 * t5641 * t883 * t21446;
    (t27860, t28022, t28069, t28072, t28073, t28075, t28079)
}
