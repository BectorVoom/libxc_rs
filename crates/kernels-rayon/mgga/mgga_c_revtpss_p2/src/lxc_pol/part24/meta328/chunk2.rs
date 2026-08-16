//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1141/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1141(t225: f64, t22964: f64, t1903: f64, t6918: f64, t4076: f64, t6895: f64, t9657: f64, t13727: f64, t1424: f64, t213: f64, t22400: f64, t22405: f64, t22407: f64, t22410: f64, t561: f64, t5715: f64, t6896: f64, t9639: f64, t9650: f64, t9666: f64, t9691: f64, t9694: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22965 = t22964 * t225;
    let t22970 = t1903 * t6918;
    let t22971 = t4076 * t22970;
    let t22974 = t6895 * t1903;
    let t22975 = t9657 * t22974;
    let t22984 = t9639 + t9650 + 0.65854491829355115987e0_f64 * t213 * t22965 * t561 - 0.19514881078765566038e-2_f64 * t13727 + 0.39512695097613069591e1_f64 * t1424 * t22971 - 0.39512695097613069591e1_f64 * t1424 * t22975 - t9666 + 0.39512695097613069591e1_f64 * t5715 * t6896 - 0.29272321618148349057e-1_f64 * t22400 + 0.29272321618148349057e-1_f64 * t22405 - 0.58544643236296698113e-1_f64 * t22407 + 0.16463622957338778996e-1_f64 * t22410 - t9691 + t9694;
    (t22965, t22970, t22971, t22974, t22975, t22984)
}
