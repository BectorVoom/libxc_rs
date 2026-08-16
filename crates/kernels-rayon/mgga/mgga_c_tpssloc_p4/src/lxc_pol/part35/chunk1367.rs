//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1367/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1367(t105445: f64, t105449: f64, t105453: f64, t105462: f64, t1528: f64, t17090: f64, t1912: f64, t28307: f64, t4147: f64, t67344: f64, t7538: f64, t82123: f64, t82154: f64, t98166: f64, t98322: f64) -> f64 {
    let t105466 = -t82123 - 0.49348022005446793095e-1_f64 * t105445 + 0.14804406601634037928e0_f64 * t105449 - 0.16449340668482264365e-1_f64 * t105453 + 0.24674011002723396548e-1_f64 * t98322 + 12.0_f64 * t4147 * t28307 - t82154 - 3.0_f64 * t17090 * t7538 - t67344 * t1912 + 0.49348022005446793095e-1_f64 * t105462 - 3.0_f64 * t98166 * t1528;
    t105466
}
