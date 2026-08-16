//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1080/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1080(t225: f64, t7723: f64, t2015: f64, t5353: f64, t3887: f64, t22897: f64, t5336: f64, t1992: f64, t22751: f64, t7733: f64, t1799: f64, t22881: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26366 = t7723 * t225;
    let t26370 = t2015 * t5353;
    let t26371 = t3887 * t26370;
    let t26378 = t22897 * t5336;
    let t26379 = t1992 * t26378;
    let t26381 = t22751 * t7733;
    let t26384 = t22881 * t1799;
    (t26366, t26370, t26371, t26379, t26381, t26384)
}
