//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2157/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2157(t25054: f64, t81651: f64, t82074: f64, t13042: f64, t13065: f64, t13463: f64, t1912: f64, t25188: f64, t25200: f64, t25348: f64, t2713: f64, t2718: f64, t2720: f64, t2743: f64, t4300: f64, t47585: f64, t6632: f64, t6662: f64, t6663: f64, t855: f64, t87861: f64, t87866: f64) -> f64 {
    let t87873 = t81651 * t82074 * t25054;
    let t87874 = 0.16449340668482264365e-1_f64 * t87873;
    let t87880 = 4.0_f64 * t855 * t2718 * t6662 * t4300 + 2.0_f64 * t25348 * t2720 - 0.3289868133696452873e-1_f64 * t87861 - t47585 * t1912 - 0.9869604401089358619e-1_f64 * t87866 - 2.0_f64 * t13042 * t6663 + 4.0_f64 * t2713 * t25200 - t87874 + 4.0_f64 * t13065 * t6632 - 2.0_f64 * t13463 * t6663 - t25188 * t2743;
    t87880
}
