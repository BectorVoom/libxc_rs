//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 568/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk568(t1503: f64, t8232: f64, t1882: f64, t6355: f64, t6280: f64, t6289: f64, t1497: f64, t2399: f64, t89: f64, t6347: f64, t870: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25194 = 4.0_f64 / 27.0_f64 * t8232 * t1503;
    let t25195 = t1882 * t6355;
    let t25246 = t1882 * t6280;
    let t25248 = t1882 * t6289;
    let t25252 = 4.0_f64 / 27.0_f64 * t89 * t2399 * t1497;
    let t25253 = t6347 * t870;
    (t25194, t25195, t25246, t25248, t25252, t25253)
}
