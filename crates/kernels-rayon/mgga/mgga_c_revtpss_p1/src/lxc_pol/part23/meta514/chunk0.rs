//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2017/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2017(t17235: f64, t19661: f64, t1042: f64, t1235: f64, t1238: f64, t1252: f64, t1261: f64, t17505: f64, t17569: f64, t21063: f64, t21085: f64, t21088: f64, t21091: f64, t21095: f64, t21102: f64, t21107: f64, t3667: f64, t5279: f64, t5320: f64, t5327: f64, t5384: f64, t6647: f64) -> (f64, f64, f64) {
    let t21110 = t17235 * t19661;
    let t21111 = t1042 * t21110;
    let t21114 = -0.42874018118069736972e-3_f64 * t5327 * t5320 + 0.22866142996303859718e-2_f64 * t21063 * t1238 - 0.21437009059034868486e-3_f64 * t3667 * t6647 - 0.21437009059034868486e-3_f64 * t1235 * t21085 + 0.15244095330869239812e-2_f64 * t21088 - 0.19055119163586549765e-3_f64 * t21091 - 0.28582678745379824648e-3_f64 * t5384 * t21095 - 0.15244095330869239812e-2_f64 * t17505 * t5279 + 0.72409452821628889107e-2_f64 * t21102 * t1252 + 0.28582678745379824648e-3_f64 * t17569 * t5279 - 0.22866142996303859718e-2_f64 * t21107 * t1252 - 0.63517063878621832552e-3_f64 * t1261 * t21111;
    (t21110, t21111, t21114)
}
