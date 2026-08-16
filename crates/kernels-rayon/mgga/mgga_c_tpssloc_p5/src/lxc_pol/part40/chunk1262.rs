//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1262/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1262(t1375: f64, t1386: f64, t16022: f64, t16460: f64, t1843: f64, t20038: f64, t20040: f64, t20044: f64, t20048: f64, t20051: f64, t20060: f64, t3758: f64, t3882: f64, t5215: f64, t5326: f64, t5354: f64, t568: f64, t6440: f64, t6461: f64) -> f64 {
    let t20062 = -6.0_f64 * t1375 * t20051 - t1386 * t20044 - t1386 * t20060 - 2.0_f64 * t16022 * t1843 - 2.0_f64 * t16460 * t1843 + t20038 * t568 + t20040 * t568 + t20048 * t568 + 2.0_f64 * t3758 * t6440 - t3758 * t6461 + 2.0_f64 * t3882 * t6440 + 4.0_f64 * t5215 * t5326 - 2.0_f64 * t5215 * t5354;
    t20062
}
