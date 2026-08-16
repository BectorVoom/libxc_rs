//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1340/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1340(t5053: f64, t2549: f64, t2569: f64, t277: f64, t39288: f64, t3980: f64, t49803: f64, t49808: f64, t5059: f64, t56954: f64, t56957: f64, t57022: f64, t57113: f64, t57117: f64, t57120: f64, t57185: f64, t57213: f64, t914: f64, t95: f64, t999: f64) -> f64 {
    let t58190 = t5053 * t5053;
    let t58195 = t56954 + t56957 + t57113 + t57117 + 0.31013857721884116596e-1_f64 * t3980 * t39288 * t5059 + 8.0_f64 * t999 * t914 * t2549 * t57022 + t57120 - 4.0_f64 / 3.0_f64 * t49803 + 4.0_f64 / 3.0_f64 * t49808 - t57185 + t57213 - 0.77534644304710291488e-2_f64 * t95 * t277 * t58190 * t2569;
    t58195
}
