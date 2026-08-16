//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 751/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk751(t109: f64, t3972: f64, t1620: f64, t410: f64, t1468: f64, t1297: f64, t1314: f64, t1301: f64, t1306: f64, t1310: f64, t193: f64, t202: f64, t210: f64, t3951: f64, t3957: f64, t3961: f64, t3963: f64, t3965: f64, t3969: f64) -> (f64, f64, f64, f64) {
    let t3973 = t109 * t3972;
    let t3974 = t1620 * t410;
    let t3977 = t1468 * t3972;
    let t3978 = t3977 * t1620;
    let t3981 = t1314 * t1297;
    let t3984 = 0.37552696856994557333e-1_f64 * t193 * t3951 * t202 - 0.35400808369803607838e-3_f64 * t1301 * t3957 * t1306 + 0.80569443951744882604e-6_f64 * t3961 * t3963 * t3965 - 40.0_f64 / 9.0_f64 * t1310 * t3969 + 50.0_f64 / 9.0_f64 * t3973 * t3974 + 50.0_f64 / 9.0_f64 * t210 * t3978 - 40.0_f64 / 9.0_f64 * t210 * t3981;
    (t3974, t3978, t3981, t3984)
}
