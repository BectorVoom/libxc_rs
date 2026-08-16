//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1150/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1150(t6703: f64, t6768: f64, t30920: f64, t3216: f64, t11094: f64, t8409: f64, t43637: f64, t8413: f64, t31003: f64, t39054: f64, t31016: f64, t9231: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t113619 = t6703 * t6768;
    let t113633 = t30920 * t3216;
    let t113637 = t8409 * t11094;
    let t113655 = t8413 * t43637;
    let t113845 = t39054 * t31003;
    let t113848 = t9231 * t31016;
    (t113619, t113633, t113637, t113655, t113845, t113848)
}
