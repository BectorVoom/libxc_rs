//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2688/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2688(t20602: f64, t225: f64, t20420: f64, t1323: f64, t1375: f64, t1385: f64, t1386: f64, t16030: f64, t16439: f64, t1807: f64, t1843: f64, t20009: f64, t20023: f64, t20025: f64, t20601: f64, t20661: f64, t20662: f64, t26224: f64, t3882: f64, t3887: f64, t5215: f64, t539: f64, t55118: f64, t56596: f64, t56607: f64, t568: f64, t6440: f64, t6461: f64, t74837: f64) -> f64 {
    let t74849 = t20602 * t225;
    let t74860 = t20420 * t225;
    let t74868 = 2.0_f64 * t1375 * t1385 * t20661 * t3887 + t1323 * t20601 * t568 + 3.0_f64 * t1807 * t20009 * t568 - 18.0_f64 * t20025 * t26224 * t55118 + t539 * t568 * t74837 - t1386 * t74849 - 3.0_f64 * t1386 * t74860 + 6.0_f64 * t16030 * t6440 - 3.0_f64 * t16439 * t6461 - 3.0_f64 * t1843 * t56596 - 6.0_f64 * t1843 * t56607 - 3.0_f64 * t20023 * t5215 - t20662 * t3882;
    t74868
}
