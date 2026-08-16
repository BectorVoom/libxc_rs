//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2690/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2690(t20672: f64, t225: f64, t1372: f64, t1386: f64, t16022: f64, t16460: f64, t1843: f64, t20044: f64, t20060: f64, t20594: f64, t20609: f64, t20613: f64, t3758: f64, t3882: f64, t5210: f64, t5326: f64, t562: f64, t56434: f64, t56580: f64, t568: f64, t6434: f64, t6440: f64, t6461: f64, t74767: f64) -> f64 {
    let t74908 = t20672 * t225;
    let t74929 = t1372 * t20594 * t568 + 3.0_f64 * t5210 * t568 * t6434 + t562 * t568 * t74767 - 3.0_f64 * t1386 * t74908 + 6.0_f64 * t16022 * t6440 + 6.0_f64 * t16460 * t6440 - 3.0_f64 * t16460 * t6461 - 3.0_f64 * t1843 * t56434 - 3.0_f64 * t1843 * t56580 + 6.0_f64 * t20044 * t5326 + 6.0_f64 * t20060 * t5326 - 6.0_f64 * t20609 * t3758 + 6.0_f64 * t20613 * t3882;
    t74929
}
