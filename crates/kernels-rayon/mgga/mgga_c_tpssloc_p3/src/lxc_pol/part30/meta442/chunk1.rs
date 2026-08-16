//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1693/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1693(t22643: f64, t6890: f64, t22642: f64, t225: f64, t6911: f64, t1372: f64, t214: f64) -> (f64, f64, f64, f64) {
    let t22644 = t22643 * t6890;
    let t22645 = t22642 * t22644;
    let t22646 = 0.82246703342411321824e-2_f64 * t22645;
    let t22656 = t6911 * t225;
    let t22666 = t214 * t1372;
    (t22644, t22646, t22656, t22666)
}
