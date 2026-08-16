//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2687/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2687(t54432: f64, t54434: f64, t39596: f64, t39601: f64, t19644: f64, t225: f64, t20038: f64, t5353: f64, t12030: f64, t12444: f64, t1323: f64, t1372: f64, t1375: f64, t1385: f64, t1386: f64, t1843: f64, t19804: f64, t20009: f64, t20022: f64, t20023: f64, t20026: f64, t20029: f64, t3758: f64, t3882: f64, t3887: f64, t3912: f64, t53866: f64, t54825: f64, t55069: f64, t55150: f64, t568: f64, t6440: f64, t6461: f64) -> (f64, f64, f64, f64, f64) {
    let t56411 = 120.0_f64 * t54432;
    let t56412 = 0.10389515463408878255e3_f64 * t54434;
    let t56416 = 192.0_f64 * t39596;
    let t56417 = 8.0_f64 * t39601;
    let t56422 = t19644 * t225;
    let t56434 = t20038 * t225;
    let t56443 = t5353 * t5353;
    let t56457 = 4.0_f64 * t1375 * t1385 * t20022 * t3887 + 2.0_f64 * t1323 * t20009 * t568 + 2.0_f64 * t1372 * t19804 * t568 + 4.0_f64 * t1375 * t3887 * t56443 - t12030 * t6461 + 4.0_f64 * t12444 * t6440 - 2.0_f64 * t12444 * t6461 - 4.0_f64 * t1386 * t56422 - 2.0_f64 * t1386 * t56434 - 4.0_f64 * t1843 * t53866 - 2.0_f64 * t1843 * t54825 - 2.0_f64 * t1843 * t55069 - 2.0_f64 * t1843 * t55150 - 2.0_f64 * t20023 * t3758 + 4.0_f64 * t20026 * t3882 - 2.0_f64 * t20029 * t3912;
    (t56411, t56412, t56416, t56417, t56457)
}
