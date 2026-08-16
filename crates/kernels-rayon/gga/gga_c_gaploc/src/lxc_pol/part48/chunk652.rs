//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 652/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk652(t1987: f64, t3650: f64, t2936: f64, t2963: f64, t3616: f64, t779: f64, t3641: f64, t702: f64, t1035: f64, t2927: f64, t3603: f64, t296: f64, t3601: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11638 = t3650 * t1987;
    let t11641 = t2936 * t2963;
    let t11644 = t779 * t3616;
    let t11647 = t3641 * t702;
    let t11650 = t1035 * t2927;
    let t11653 = t779 * t3603;
    let t11656 = t296 * t3601;
    (t11638, t11641, t11644, t11647, t11650, t11653, t11656)
}
