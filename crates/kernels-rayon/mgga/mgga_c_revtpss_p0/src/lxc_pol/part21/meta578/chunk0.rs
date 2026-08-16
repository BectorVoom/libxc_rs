//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2285/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2285(t17879: f64, t460: f64, t3584: f64, t5457: f64, t5351: f64, t1269: f64, t3766: f64, t1280: f64, t17345: f64, t1287: f64, t17389: f64, t17600: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t17880 = t460 * t17879;
    let t17883 = t5457 * t3584;
    let t17884 = t5351 * t17883;
    let t17887 = t3766 * t1269;
    let t17888 = t460 * t17887;
    let t17893 = t1280 * t17345;
    let t17902 = t17389 * t1287;
    let t17905 = t17600 * t1287;
    (t17880, t17883, t17884, t17887, t17888, t17893, t17902, t17905)
}
