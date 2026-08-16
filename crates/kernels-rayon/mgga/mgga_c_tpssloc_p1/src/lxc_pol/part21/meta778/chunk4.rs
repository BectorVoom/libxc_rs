//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2694/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2694(t56475: f64, t56525: f64, t56542: f64, t56568: f64, t20032: f64, t225: f64, t20040: f64, t12033: f64, t1386: f64, t16022: f64, t16437: f64, t16452: f64, t16453: f64, t16475: f64, t1843: f64, t20023: f64, t20029: f64, t20044: f64, t20060: f64, t26224: f64, t3752: f64, t3882: f64, t3889: f64, t3912: f64, t5215: f64, t5321: f64, t5354: f64, t55093: f64, t55118: f64, t562: f64, t568: f64, t6434: f64, t6440: f64, t6461: f64) -> (f64, f64) {
    let t56570 = t56475 + t56525 + t56542 + t56568;
    let t56580 = t20032 * t225;
    let t56596 = t20040 * t225;
    let t56605 = -24.0_f64 * t16452 * t26224 * t55118 + t3752 * t568 * t6434 + t562 * t56570 * t568 + 2.0_f64 * t12033 * t6440 - t12033 * t6461 - 2.0_f64 * t1386 * t56580 - 2.0_f64 * t1386 * t56596 - 4.0_f64 * t16022 * t5354 - 2.0_f64 * t16437 * t5321 + 8.0_f64 * t16453 * t5215 - 12.0_f64 * t16475 * t5321 - 4.0_f64 * t1843 * t55093 - 2.0_f64 * t20023 * t3882 + 4.0_f64 * t20029 * t3889 + 2.0_f64 * t20044 * t3889 - t20044 * t3912 - t20060 * t3912;
    (t56570, t56605)
}
