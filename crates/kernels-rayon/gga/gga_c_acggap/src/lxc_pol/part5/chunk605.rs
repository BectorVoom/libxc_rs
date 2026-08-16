//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 605/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk605(t175: f64, t3044: f64, t398: f64, t1036: f64, t301: f64, t879: f64) -> (f64, f64, f64) {
    let t3806 = t398 * t175 * t3044;
    let t3808 = 0.12862205435420921092e-2_f64 * t1036 * t3806;
    let t3809 = t879 * t301;
    (t3806, t3808, t3809)
}
