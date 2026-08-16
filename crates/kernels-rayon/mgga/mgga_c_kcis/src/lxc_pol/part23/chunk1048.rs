//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1048/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1048(t1386: f64, t1444: f64, t2642: f64, t5709: f64, t3964: f64, t491: f64, t990: f64) -> (f64, f64, f64, f64) {
    let t27453 = t1386 * t1444;
    let t27454 = t27453 * t2642;
    let t27455 = t5709 * t27454;
    let t27459 = t3964 * t491 * t990;
    (t27453, t27454, t27455, t27459)
}
