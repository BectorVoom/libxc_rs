//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1321/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1321(t1014: f64, t29380: f64, t2002: f64, t303: f64, t98607: f64, t29386: f64, t28524: f64, t5633: f64, t1983: f64, t5757: f64, t576: f64, t7052: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t102649 = t1014 * t29380;
    let t102653 = t303 * t98607 * t2002;
    let t102655 = t1014 * t29386;
    let t102658 = t303 * t28524 * t5633;
    let t102661 = t303 * t1983 * t5757;
    let t102664 = t576 * t7052;
    (t102649, t102653, t102655, t102658, t102661, t102664)
}
