//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2311/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2311(t24668: f64, t27497: f64, t15643: f64, t7345: f64, t27639: f64, t86264: f64, t27645: f64, t3540: f64, t8043: f64, t15545: f64, t15667: f64, t24699: f64, t24749: f64, t27655: f64, t7310: f64, t7316: f64, t8028: f64, t8035: f64, t86191: f64, t86234: f64) -> f64 {
    let t95346 = t24668 * t27497;
    let t95352 = t7345 * t15643 / 864.0_f64;
    let t95362 = 0.40372756094140390856e-3_f64 * t86264 * t27639;
    let t95364 = 0.20186378047070195428e-3_f64 * t86264 * t27645;
    let t95365 = t8043 * t3540;
    let t95367 = 0.20186378047070195428e-3_f64 * t86234 * t95346 + 5.0_f64 / 6912.0_f64 * t7345 * t15545 - t95352 + 0.20186378047070195428e-3_f64 * t7316 * t27655 + 0.10093189023535097714e-3_f64 * t24749 * t8035 - t7310 * t15667 / 288.0_f64 + 0.80745512188280781712e-3_f64 * t8028 * t24699 + t95362 - t95364 - t95365 / 6912.0_f64 + t86191;
    t95367
}
