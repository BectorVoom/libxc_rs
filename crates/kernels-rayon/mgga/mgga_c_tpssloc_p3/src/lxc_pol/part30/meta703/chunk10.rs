//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2295/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2295(t7583: f64, t88383: f64, t25650: f64, t3030: f64, t88449: f64, t1015: f64, t17714: f64, t17959: f64, t23489: f64, t23537: f64, t25652: f64, t25655: f64, t25660: f64, t25661: f64, t28566: f64, t28587: f64, t360: f64, t5866: f64, t6730: f64, t6742: f64, t6744: f64, t68: f64, t83157: f64, t88290: f64, t88407: f64, t88723: f64) -> f64 {
    let t99834 = t88383 * t7583;
    let t99848 = t25650 * t88449 * t3030;
    let t99855 = 0.10093189023535097714e-3_f64 * t23489 * t28587 + 0.10093189023535097714e-3_f64 * t6742 * t6744 * t17959 * t68 * t360 - 0.20186378047070195428e-3_f64 * t99834 + 0.20186378047070195428e-3_f64 * t88290 * t7583 - t83157 / 1296.0_f64 - 0.10093189023535097714e-3_f64 * t6730 * t28566 - 0.10093189023535097714e-3_f64 * t25652 * t1015 * t5866 * t25660 - 0.20186378047070195428e-3_f64 * t88407 * t7583 + 0.40372756094140390856e-3_f64 * t99848 * t25655 - 0.20186378047070195428e-3_f64 * t99848 * t25661 + t23537 * t17714 / 768.0_f64 + t88723;
    t99855
}
