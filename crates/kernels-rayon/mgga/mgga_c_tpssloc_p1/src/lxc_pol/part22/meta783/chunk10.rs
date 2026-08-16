//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2689/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2689(t1375: f64, t1385: f64, t16022: f64, t16030: f64, t16439: f64, t1843: f64, t20023: f64, t20026: f64, t20029: f64, t20051: f64, t20608: f64, t20613: f64, t20662: f64, t3758: f64, t3887: f64, t40591: f64, t5215: f64, t5318: f64, t5321: f64, t5353: f64, t5354: f64, t56422: f64, t568: f64, t6361: f64, t6440: f64, t6460: f64, t6461: f64) -> f64 {
    let t74899 = 24.0_f64 * t1375 * t1385 * t20608 * t40591 + 6.0_f64 * t1375 * t3887 * t5353 * t6460 + 3.0_f64 * t5318 * t568 * t6361 - 3.0_f64 * t16022 * t6461 - 3.0_f64 * t16030 * t6461 + 6.0_f64 * t16439 * t6440 - 6.0_f64 * t1843 * t56422 - 3.0_f64 * t20023 * t5321 + 6.0_f64 * t20026 * t5321 - 6.0_f64 * t20029 * t5354 - 18.0_f64 * t20051 * t5215 + 6.0_f64 * t20613 * t3758 - t20662 * t3758;
    t74899
}
