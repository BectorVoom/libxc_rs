//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1064/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1064(t12231: f64, t1598: f64, t12234: f64, t498: f64, t18210: f64, t7915: f64, t2237: f64, t7900: f64) -> (f64, f64, f64, f64, f64) {
    let t27339 = t12231 * t1598;
    let t27340 = t498 * t12234;
    let t27345 = t18210 * t7915;
    let t27346 = t2237 * t27345;
    let t27348 = t18210 * t7900;
    (t27339, t27340, t27345, t27346, t27348)
}
