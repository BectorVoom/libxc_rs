//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1297/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1297(t1862: f64, t8308: f64, t31688: f64, t31693: f64, t31687: f64, t8515: f64, t9231: f64, t31019: f64, t2240: f64, t240: f64, t8301: f64, t39054: f64, t8511: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t115833 = t8308 * t1862;
    let t115837 = t31688 * t31693;
    let t115846 = t9231 * t31687 * t8515;
    let t115853 = t31688 * t31019;
    let t115860 = 55.0_f64 / 81.0_f64 * t2240 * t8301 * t240 * t8515;
    let t115866 = t39054 * t8511;
    (t115833, t115837, t115846, t115853, t115860, t115866)
}
