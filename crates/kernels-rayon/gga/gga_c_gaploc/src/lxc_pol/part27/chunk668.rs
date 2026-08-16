//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 668/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk668(t2084: f64, t296: f64, t120: f64, t19: f64, t320: f64, t2088: f64, t298: f64) -> (f64, f64, f64, f64) {
    let t5745 = t2084 * t296;
    let t5746 = t120 * t5745;
    let t5747 = t5746 * t19;
    let t5748 = t320 * t5747;
    let t5750 = 1.0_f64 / t2088 / t298;
    (t5745, t5746, t5748, t5750)
}
