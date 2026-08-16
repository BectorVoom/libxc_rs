//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 474/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk474(t486: f64, t1371: f64, t1650: f64, t1370: f64, t1924: f64) -> (f64, f64, f64) {
    let t495 = 0.0_f64 < t486;
    let t1933 = t1371 * t1650;
    let t1934 = t1370 * t1933;
    let t1938 = piecewise3(t495, t1924, -t1924);
    (t1933, t1934, t1938)
}
