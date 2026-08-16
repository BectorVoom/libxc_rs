//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1181/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1181(t1349: f64, t139192: f64, t147930: f64, t148166: f64, t148234: f64, t149093: f64, t149120: f64, t149141: f64, t24080: f64, t26568: f64, t28: f64, t34967: f64, t35016: f64, t558: f64, t5766: f64, t5772: f64, t5778: f64, t5973: f64, t6616: f64, t6723: f64) -> f64 {
    let t149458 = -t5766 * t35016 / 3.0_f64 + 2.0_f64 / 9.0_f64 * t5772 * t24080 * t26568 + t139192 / 54.0_f64 + t5766 * t34967 / 6.0_f64 - 2.0_f64 / 3.0_f64 * t1349 * t28 * t5778 * t6723 * t558 + 8.0_f64 * t147930 + 8.0_f64 * t148166 + 8.0_f64 * t149120 + t1349 * t28 * t6616 * t5973 / 3.0_f64 + 8.0_f64 * t149141 - 12.0_f64 * t148234 + 4.0_f64 * t149093;
    t149458
}
