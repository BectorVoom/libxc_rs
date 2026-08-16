//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1368/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1368(t1364: f64, t14029: f64, t14245: f64, t1692: f64, t1812: f64, t18728: f64, t18812: f64, t19818: f64, t20510: f64, t21659: f64, t2439: f64, t3552: f64, t4701: f64, t4806: f64, t51780: f64, t52613: f64, t5849: f64, t5853: f64, t62807: f64, t62829: f64, t66299: f64, t69810: f64, t69847: f64, t69863: f64, t69881: f64, t70240: f64, t70243: f64, t70759: f64, t750: f64) -> f64 {
    let t72411 = 6.0_f64 * t1364 * t20510 * t2439 + 3.0_f64 * t14029 * t1812 * t2439 + 12.0_f64 * t14245 * t1812 * t3552 + 4.0_f64 * t1692 * t18812 * t69881 + 2.0_f64 * t1692 * t18812 * t70240 + 4.0_f64 * t1692 * t19818 * t66299 + 2.0_f64 * t1692 * t4806 * t62829 - 6.0_f64 * t1692 * t62807 * t70243 + 6.0_f64 * t18812 * t2439 * t69847 + 3.0_f64 * t21659 * t2439 * t750 + 3.0_f64 * t2439 * t4701 * t5849 - 3.0_f64 * t2439 * t52613 * t5853 - 6.0_f64 * t2439 * t5853 * t69810 - 3.0_f64 * t2439 * t5853 * t69863 - 6.0_f64 * t3552 * t51780 * t5853 + 12.0_f64 * t18728 * t70759;
    t72411
}
