//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2475/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2475(t10422: f64, t21519: f64, t3070: f64, t10403: f64, t10408: f64, t10904: f64, t21487: f64, t49662: f64, t5677: f64, t61916: f64, t61919: f64, t61923: f64, t61929: f64, t61940: f64, t61975: f64, t61977: f64, t70082: f64) -> f64 {
    let t70404 = t3070 * t10422 * t21519;
    let t70414 = 5.0_f64 / 2304.0_f64 * t10403 * t10408 * t5677 * t70082 + 5.0_f64 / 6912.0_f64 * t61916 - t70404 / 1152.0_f64 - t61919 / 576.0_f64 - 5.0_f64 / 1152.0_f64 * t61923 + t61929 / 1152.0_f64 - t10904 * t21487 / 96.0_f64 - t49662 + t61940 / 1152.0_f64 - t61975 / 1536.0_f64 + t61977 / 2304.0_f64;
    t70414
}
