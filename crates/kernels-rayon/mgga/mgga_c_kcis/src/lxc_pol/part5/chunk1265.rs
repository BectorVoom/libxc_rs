//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1265/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1265(t1368: f64, t16842: f64, t16845: f64, t21061: f64, t21065: f64, t21069: f64, t21074: f64, t21079: f64, t21084: f64, t21088: f64, t21098: f64, t5691: f64, t5702: f64, t5706: f64, t5710: f64) -> f64 {
    let t21101 = 11.0_f64 / 324.0_f64 * t21061 - t1368 * t21065 / 288.0_f64 - t1368 * t21069 / 288.0_f64 - t1368 * t21074 / 144.0_f64 + t1368 * t21079 / 216.0_f64 + t1368 * t21084 / 144.0_f64 - t21088 / 432.0_f64 + t5691 * t5706 / 54.0_f64 + t5691 * t5710 / 27.0_f64 - 2.0_f64 / 81.0_f64 * t5691 * t5702 + t16842 / 216.0_f64 + t16845 + t1368 * t21098 / 72.0_f64;
    t21101
}
