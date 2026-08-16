//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2772/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2772(t10489: f64, t10618: f64, t10635: f64, t10818: f64, t14468: f64, t14643: f64, t14648: f64, t14649: f64, t14652: f64, t1553: f64, t1555: f64, t225: f64, t227: f64, t229: f64, t2394: f64, t2430: f64, t2639: f64, t4409: f64, t4415: f64, t4416: f64, t50151: f64, t50391: f64, t50844: f64, t50845: f64, t50847: f64, t50848: f64, t50851: f64, t50854: f64, t50882: f64, t50908: f64, t775: f64, t832: f64, t853: f64) -> f64 {
    let t50914 = 180.0_f64 * t4415 * t14648 * t10818 + 3.0_f64 * t227 * t832 * t50151 + 180.0_f64 * t4415 * t50391 * t2394 - 36.0_f64 * t4415 * t853 * t14468 * t775 - 36.0_f64 * t4415 * t14652 * t2430 + 3.0_f64 * t1553 * t10635 + 3.0_f64 * t10618 * t1555 - 12.0_f64 * t4415 * t4416 * t10489 - 36.0_f64 * t4409 * t2639 + 180.0_f64 * t14643 * t14649 - (t50844 + t50845 + t50847 + t50848 + t50851 + t50854 + t50882 + t50908) * t225 * t229;
    t50914
}
