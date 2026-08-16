//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1363/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1363(t31590: f64, t493: f64, t1441: f64, t590: f64, t2897: f64, t4398: f64, t7030: f64, t1415: f64, t8330: f64, t2365: f64, t25740: f64, t7025: f64) -> (f64, f64, f64, f64) {
    let t34273 = t493 * t31590;
    let t34276 = 0.2044956050875773316e1_f64 * t1441 * t34273 * t590;
    let t34278 = t4398 * t2897 * t7030;
    let t34279 = 0.29792074959875355558e-1_f64 * t34278;
    let t34281 = t1415 * t8330 * t7030;
    let t34282 = 0.29792074959875355558e-1_f64 * t34281;
    let t34284 = t7025 * t2365 * t25740;
    (t34276, t34279, t34282, t34284)
}
