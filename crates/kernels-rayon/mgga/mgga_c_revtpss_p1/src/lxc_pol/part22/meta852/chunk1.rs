//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2994/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2994(t14322: f64, t2516: f64, t2496: f64, t14426: f64, t177: f64, t762: f64, t10428: f64, t4305: f64, t2609: f64, t4186: f64, t706: f64, t10436: f64, t4311: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t49957 = t14322 * t2516;
    let t49963 = t14322 * t2496;
    let t49966 = t14426 * t177 * t762;
    let t49978 = t10428 * t4305;
    let t49981 = t706 * t2609 * t4186;
    let t49983 = t4311 * t10436;
    (t49957, t49963, t49966, t49978, t49981, t49983)
}
