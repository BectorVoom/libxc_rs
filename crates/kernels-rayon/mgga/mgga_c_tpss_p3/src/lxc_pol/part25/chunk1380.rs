//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1380/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1380(t1673: f64, t6441: f64, t1276: f64, t1278: f64, t1284: f64, t16041: f64, t16079: f64, t1666: f64, t1849: f64, t1856: f64, t20697: f64, t21948: f64, t21984: f64, t4562: f64, t5466: f64, t5480: f64, t5942: f64, t5960: f64, t6442: f64, t67849: f64, t67851: f64, t67853: f64, t71181: f64, t72733: f64) -> f64 {
    let t72737 = t6441 * t1673;
    let t72743 = 2.0_f64 * t6442 * t4562 + t21948 * t1284 + 2.0_f64 * t1666 * t20697 + t1278 * (t71181 + t72733) + t67849 + t16041 * t1856 + t67851 + 2.0_f64 * t72737 + t67853 + t5942 * t5480 + t5466 * t5960 + t1276 * t21984 + t1849 * t16079;
    t72743
}
