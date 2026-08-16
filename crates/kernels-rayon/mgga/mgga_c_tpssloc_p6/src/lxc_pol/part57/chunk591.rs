//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 591/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk591(t6799: f64, t7610: f64, t1625: f64, t1948: f64, t345: f64, t1615: f64, t1945: f64, t1060: f64, t383: f64, t7593: f64, t1058: f64, t1610: f64, t1920: f64, t1953: f64, t353: f64, t6687: f64, t6783: f64, t6797: f64, t7604: f64, t7607: f64) -> (f64, f64, f64, f64, f64) {
    let t7611 = t6799 * t7610;
    let t7614 = t1948 * t1625;
    let t7615 = t345 * t7614;
    let t7619 = t1945 * t1615;
    let t7620 = t7619 * t1060;
    let t7622 = t383 * t7593;
    let t7624 = t6783 + 0.27415567780803773942e-2_f64 * t6687 * t7604 - 0.82246703342411321825e-2_f64 * t6687 * t7607 + 0.82246703342411321825e-2_f64 * t6797 * t7611 + 0.82246703342411321825e-2_f64 * t1920 * t7615 + t1610 * t1953 + t1058 * t7620 + t353 * t7622;
    (t7611, t7614, t7620, t7622, t7624)
}
