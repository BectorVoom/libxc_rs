//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 667/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk667(t533: f64, t8639: f64, t1390: f64, t1983: f64, t2018: f64, t3701: f64, t2095: f64, t1873: f64, t7230: f64, t2039: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8640 = t533 * t8639;
    let t8641 = t8640 * t1390;
    let t8642 = t1983 * t8641;
    let t8643 = t3701 * t2018;
    let t8644 = t2095 * t8643;
    let t8645 = t1983 * t8644;
    let t8654 = 0.135e2_f64 * t7230 * t1873;
    let t8657 = t2039 * t1873;
    (t8640, t8641, t8642, t8643, t8644, t8645, t8654, t8657)
}
