//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1332/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1332(t1000: f64, t10615: f64, t13632: f64, t13912: f64, t1435: f64, t16887: f64, t16891: f64, t16895: f64, t16919: f64, t3608: f64, t4038: f64, t4054: f64, t49707: f64, t49754: f64, t5065: f64, t5069: f64, t56939: f64, t56941: f64, t57046: f64, t57628: f64, t914: f64, t999: f64) -> f64 {
    let t58027 = -4.0_f64 * t13632 * t16919 + t56939 - 16.0_f64 / 9.0_f64 * t49707 - 8.0_f64 * t4038 * t3608 * t10615 * t57628 + t56941 + 2.0_f64 / 3.0_f64 * t49754 * t1435 + 4.0_f64 * t4054 * t16887 + t999 * t914 * t1000 * t57046 / 6.0_f64 - 16.0_f64 / 3.0_f64 * t4054 * t16891 + 2.0_f64 / 3.0_f64 * t4054 * t16895 + t13912 * t5065 + 4.0_f64 / 3.0_f64 * t13912 * t5069;
    t58027
}
