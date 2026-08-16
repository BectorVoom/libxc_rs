//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2245/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2245(t13034: f64, t225: f64, t10104: f64, t10116: f64, t13029: f64, t13042: f64, t13050: f64, t13072: f64, t13460: f64, t13461: f64, t13463: f64, t1528: f64, t259: f64, t2597: f64, t2713: f64, t2718: f64, t2720: f64, t2743: f64, t40870: f64, t4147: f64, t4273: f64, t852: f64, t855: f64, t865: f64, t866: f64, t9590: f64) -> f64 {
    let t46452 = t13034 * t225;
    let t46481 = 6.0_f64 * t13460 * t2718 * t855 * t865 + 3.0_f64 * t13029 * t259 * t852 - t10104 * t4147 + 6.0_f64 * t10116 * t4147 + 6.0_f64 * t13042 * t2720 - 3.0_f64 * t13042 * t2743 - 18.0_f64 * t13050 * t2597 + 12.0_f64 * t13072 * t2713 - 3.0_f64 * t13461 * t2713 - 3.0_f64 * t13463 * t2743 - 3.0_f64 * t1528 * t40870 + 6.0_f64 * t4273 * t9590 - 3.0_f64 * t46452 * t866;
    t46481
}
