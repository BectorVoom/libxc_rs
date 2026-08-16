//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1218/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1218(t28552: f64, t37467: f64, t37470: f64, t28559: f64, t28561: f64, t16287: f64, t1879: f64, t2198: f64, t22296: f64, t3593: f64, t4611: f64, t4744: f64, t48045: f64, t48051: f64, t55893: f64, t95: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t56012 = 384.0_f64 * t28552;
    let t56013 = 48.0_f64 * t37467;
    let t56014 = 6.0_f64 * t37470;
    let t56015 = 144.0_f64 * t28559;
    let t56016 = 48.0_f64 * t28561;
    let t56024 = 0.46520786582826174894e-1_f64 * t95 * t2198 * t55893 + t56012 - t56013 + t56014 + t56015 - t56016 + t22296 + 6.0_f64 * t48045 + 6.0_f64 * t48051 + 0.31013857721884116596e-1_f64 * t1879 * t3593 * t16287 + 3.0_f64 * t4611 * t4744;
    (t56012, t56013, t56014, t56015, t56016, t56024)
}
