//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2970/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2970(t17997: f64, t3070: f64, t42488: f64, t1041: f64, t13969: f64, t17975: f64, t10413: f64, t10876: f64, t10937: f64, t14080: f64, t1409: f64, t14167: f64, t14172: f64, t14218: f64, t14219: f64, t17649: f64, t17712: f64, t17920: f64, t17923: f64, t3071: f64, t3131: f64, t3132: f64, t3966: f64, t42483: f64, t43361: f64, t4579: f64, t4582: f64, t4590: f64, t4644: f64, t49604: f64, t49607: f64, t49621: f64, t49629: f64, t49984: f64, t61910: f64, t883: f64) -> f64 {
    let t61916 = t3070 * t42488 * t17997;
    let t61919 = t1041 * t13969 * t17975;
    let t61921 = t10937 * t17649 / 216.0_f64 + t49604 / 1728.0_f64 + t49607 / 1728.0_f64 - t49984 * t4579 / 216.0_f64 - t10876 * t4582 * t17712 * t3132 / 512.0_f64 - t10413 * t3071 * t14218 * t14219 * t3966 / 1152.0_f64 - t43361 * t3071 * t49621 * t3131 * t883 * t1409 / 384.0_f64 + t42483 * t3071 * t49621 * t17923 / 2304.0_f64 - 5.0_f64 / 648.0_f64 * t10937 * t17920 + t49629 / 864.0_f64 + t4644 * t14167 / 384.0_f64 - 5.0_f64 / 648.0_f64 * t14080 * t4590 - 5.0_f64 / 2304.0_f64 * t1041 * t4582 * t14172 * t61910 + 5.0_f64 / 10368.0_f64 * t61916 - t61919 / 864.0_f64;
    t61921
}
