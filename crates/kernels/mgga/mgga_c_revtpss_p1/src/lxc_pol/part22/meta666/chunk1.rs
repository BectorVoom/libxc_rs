//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2627/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2627<F: Float>(t1042: F, t21094: F, t1038: F, t6593: F, t1244: F, t1241: F, t5273: F, t5292: F, t17235: F, t19661: F, t1235: F, t1238: F, t1252: F, t1261: F, t17505: F, t17569: F, t21063: F, t21085: F, t21088: F, t21091: F, t3667: F, t5279: F, t5320: F, t5327: F, t5384: F, t6647: F) -> (F, F, F, F, F, F, F) {
    let t21095 = t1042 * t21094;
    let t21100 = t6593 * t1038;
    let t21101 = t1244 * t21100;
    let t21102 = t1241 * t21101;
    let t21107 = t5273 * t5292;
    let t21110 = t17235 * t19661;
    let t21111 = t1042 * t21110;
    let t21114 = -F::cast_from(0.42874018118069736972e-3_f64) * t5327 * t5320 + F::cast_from(0.22866142996303859718e-2_f64) * t21063 * t1238 - F::cast_from(0.21437009059034868486e-3_f64) * t3667 * t6647 - F::cast_from(0.21437009059034868486e-3_f64) * t1235 * t21085 + F::cast_from(0.15244095330869239812e-2_f64) * t21088 - F::cast_from(0.19055119163586549765e-3_f64) * t21091 - F::cast_from(0.28582678745379824648e-3_f64) * t5384 * t21095 - F::cast_from(0.15244095330869239812e-2_f64) * t17505 * t5279 + F::cast_from(0.72409452821628889107e-2_f64) * t21102 * t1252 + F::cast_from(0.28582678745379824648e-3_f64) * t17569 * t5279 - F::cast_from(0.22866142996303859718e-2_f64) * t21107 * t1252 - F::cast_from(0.63517063878621832552e-3_f64) * t1261 * t21111;
    (t21095, t21101, t21102, t21107, t21110, t21111, t21114)
}
