//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 756/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk756(t3426: f64, t3972: f64, t3931: f64, t2464: f64, t969: f64, t3758: f64, t970: f64, t242: f64, t1471: f64, t2652: f64, t2660: f64, t2678: f64, t2731: f64, t2740: f64, t2748: f64, t2754: f64, t3952: f64, t3956: f64, t3963: f64, t3970: f64, t946: f64, t967: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3973 = t3972 * t3426;
    let t3974 = t3931 * t3973;
    let t3977 = t969 * t2464;
    let t3978 = t3977 * t3426;
    let t3979 = t3931 * t3978;
    let t3982 = t970 * t3758;
    let t3983 = t242 * t3982;
    let t3986 = t946 * t3952 / 3072.0_f64 - t2731 * t3956 / 3072.0_f64 + t2678 / 4608.0_f64 - t2660 / 864.0_f64 - t2652 + t2754 / 6912.0_f64 + t2740 * t3963 / 4608.0_f64 - t2748 * t1471 / 864.0_f64 + t3970 / 6912.0_f64 + 5.0_f64 / 13824.0_f64 * t967 * t3974 - t967 * t3979 / 2304.0_f64 + t967 * t3983 / 4608.0_f64;
    (t3973, t3974, t3977, t3978, t3979, t3983, t3986)
}
