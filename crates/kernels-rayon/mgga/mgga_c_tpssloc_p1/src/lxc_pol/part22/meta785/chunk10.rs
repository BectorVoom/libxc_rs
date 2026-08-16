//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2713/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2713(t12757: f64, t19473: f64, t19529: f64, t20304: f64, t20342: f64, t2331: f64, t29903: f64, t4043: f64, t4067: f64, t45435: f64, t5488: f64, t55420: f64, t64: f64, t656: f64, t666: f64, t75592: f64, t75601: f64, t75603: f64, t75613: f64, t75657: f64, t75694: f64) -> f64 {
    let t75699 = t55420 + 2.0_f64 * t75592 + 3.0_f64 * t64 * t45435 * t20304 * t666 - 9.0_f64 / 4.0_f64 * t64 * t19473 * t4067 - 2.0_f64 * t75601 - 9.0_f64 / 4.0_f64 * t29903 * t75603 * t666 + 3.0_f64 / 4.0_f64 * t64 * t12757 * t5488 + 3.0_f64 / 4.0_f64 * t64 * t4043 * t19529 + t75613 / 3.0_f64 + t64 * t2331 * t20342 * t666 / 4.0_f64 - t64 * t656 * (t75657 + t75694) / 8.0_f64;
    t75699
}
