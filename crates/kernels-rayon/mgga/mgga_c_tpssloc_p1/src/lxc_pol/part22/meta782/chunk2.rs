//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2674/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2674(t1348: f64, t1821: f64, t19702: f64, t19708: f64, t19716: f64, t19719: f64, t19725: f64, t20536: f64, t225: f64, t5272: f64, t5280: f64, t5283: f64, t548: f64, t550: f64, t6404: f64, t6408: f64, t6411: f64, t68: f64, t74466: f64, t74467: f64, t74469: f64, t74471: f64, t74480: f64, t74487: f64, t74498: f64, t74505: f64, t74562: f64) -> f64 {
    let t74564 = (-(t74466 + t74467 + t74469 + t74471 + t74480 + t74487 + t74498 + t74505) * t225 * t548 + 3.0_f64 * t20536 * t1348 + 9.0_f64 * t19702 * t1821 - 36.0_f64 * t6404 * t68 * t5280 + 9.0_f64 * t6404 * t5283 - 36.0_f64 * t5272 * t6408 + 180.0_f64 * t19708 * t19716 - 72.0_f64 * t19708 * t19719 + 9.0_f64 * t5272 * t6411 - 36.0_f64 * t19708 * t19725 + t74562) * t550;
    t74564
}
