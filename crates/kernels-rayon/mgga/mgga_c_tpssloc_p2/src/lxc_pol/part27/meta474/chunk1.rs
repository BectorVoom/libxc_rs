//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1840/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1840(t1046: f64, t1935: f64, t23489: f64, t23495: f64, t23500: f64, t23504: f64, t23510: f64, t23515: f64, t23521: f64, t23529: f64, t3057: f64, t3064: f64, t6723: f64, t6730: f64, t6735: f64, t6742: f64, t6747: f64, t6765: f64) -> f64 {
    let t23532 = 0.20186378047070195428e-3_f64 * t23489 * t6747 - 0.20186378047070195428e-3_f64 * t6730 * t6735 - 0.10093189023535097714e-3_f64 * t1935 * t23495 + 0.16149102437656156342e-2_f64 * t6723 * t6735 + t23500 / 1152.0_f64 + 0.10093189023535097714e-3_f64 * t6742 * t23504 + 0.20186378047070195428e-3_f64 * t23510 * t23515 - 0.10093189023535097714e-3_f64 * t23510 * t23521 + t6765 * t3057 / 2304.0_f64 + 5.0_f64 / 6912.0_f64 * t6765 * t3064 - t23529 * t1046 / 216.0_f64;
    t23532
}
