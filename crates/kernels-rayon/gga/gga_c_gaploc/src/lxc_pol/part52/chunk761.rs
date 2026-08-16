//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 761/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk761(t107: f64, t35439: f64, t787: f64, t11613: f64, t769: f64, t11822: f64, t1980: f64, t36364: f64, t1858: f64, t3601: f64, t6058: f64, t11595: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t36700 = t787 * t35439 * t107;
    let t36738 = t769 * t11613;
    let t36762 = t1980 * t11822;
    let t36782 = t787 * t36364;
    let t36798 = t1858 * t3601;
    let t37032 = t6058 * t3601;
    let t37057 = t769 * t11595;
    (t36700, t36738, t36762, t36782, t36798, t37032, t37057)
}
