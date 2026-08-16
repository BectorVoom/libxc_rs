//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 378/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk378(t1716: f64, t738: f64, t271: f64, t341: f64, t667: f64, t656: f64, t1097: f64, t19: f64, t252: f64, t1: f64, t664: f64, t1112: f64, t1114: f64, t1116: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1717 = t738 * t1716;
    let t1735 = t341 * t271;
    let t1741 = t667 * t667;
    let t1742 = t656 * t1741;
    let t1747 = t1097 * t252 * t19;
    let t1751 = t341 * t664 * t1;
    let t1759 = -0.99474444444444444447e-4_f64 * t1112 + 0.19894888888888888889e-3_f64 * t1114 + 0.52442777777777777777e-2_f64 * t1116;
    (t1717, t1735, t1741, t1742, t1747, t1751, t1759)
}
