//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1068/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1068(t242: f64, t2675: f64, t3950: f64, t946: f64, t11663: f64, t11667: f64, t11671: f64, t11675: f64, t11679: f64, t11683: f64, t11688: f64, t11692: f64, t11697: f64, t1471: f64, t2682: f64, t2740: f64, t3952: f64, t8963: f64, t967: f64) -> f64 {
    let t11701 = t242 * t2675 * t3950;
    let t11703 = t946 * t11701 / 2304.0_f64;
    let t11704 = 5.0_f64 / 5184.0_f64 * t967 * t11663 - t967 * t11667 / 1152.0_f64 - t967 * t11671 / 2304.0_f64 + t2740 * t11675 / 2304.0_f64 + t2740 * t11679 / 4608.0_f64 + 5.0_f64 / 13824.0_f64 * t2740 * t11683 - t11688 / 13824.0_f64 - t11692 / 20736.0_f64 + 19.0_f64 / 2592.0_f64 * t8963 * t1471 - t11697 - t2682 * t3952 / 288.0_f64 + t11703;
    t11704
}
