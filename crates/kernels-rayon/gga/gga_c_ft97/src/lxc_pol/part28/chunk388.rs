//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 388/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk388(t554: f64, t72: f64, t5579: f64, t1355: f64, t5608: f64, t3392: f64, t5812: f64) -> (f64, f64, f64, f64) {
    let t5830 = t72 * t554;
    let t5831 = t5579 * t5830;
    let t5837 = 0.16669500273148148149e-1_f64 * t1355 * t5608;
    let t5838 = t3392 * t5812;
    (t5830, t5831, t5837, t5838)
}
