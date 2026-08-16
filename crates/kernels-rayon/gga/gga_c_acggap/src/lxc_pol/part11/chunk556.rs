//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 556/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk556(t3266: f64, t386: f64, t388: f64, t384: f64, t1032: f64, t1103: f64, t175: f64, t3044: f64, t398: f64, t1036: f64, t301: f64, t879: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3786 = t386 * t3266 * t388;
    let t3787 = t384 * t3786;
    let t3793 = t1032 * t1103;
    let t3806 = t398 * t175 * t3044;
    let t3808 = 0.12862205435420921092e-2_f64 * t1036 * t3806;
    let t3809 = t879 * t301;
    (t3786, t3787, t3793, t3806, t3808, t3809)
}
