//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 297/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk297(t1233: f64, t1236: f64, t143: f64, t463: f64) -> (f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t1237 = t1233 * t1236;
    let t1238 = t143 * t143;
    let t1240 = 1.0_f64 / t1238 / t143;
    let t1242 = t1240 * pi * t463;
    (t1237, t1238, t1240, t1242)
}
