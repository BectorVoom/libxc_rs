//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 555/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk555(t1431: f64, t8232: f64, t1882: f64, t6172: f64, t6189: f64, t1501: f64, t668: f64, t6367: f64, t6376: f64, t1495: f64, t848: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t24815 = 4.0_f64 / 27.0_f64 * t8232 * t1431;
    let t24841 = t1882 * t6172;
    let t24843 = t1882 * t6189;
    let t24873 = t1501 * t668;
    let t24882 = t1882 * t6367;
    let t24884 = t1882 * t6376;
    let t24886 = t848 * t1495;
    (t24815, t24841, t24843, t24873, t24882, t24884, t24886)
}
