//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1153/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1153(t1354: f64, t39947: f64, t12365: f64, t3853: f64, t12267: f64, t3789: f64, t3798: f64, t12297: f64, t12385: f64, t12300: f64, t3858: f64, t12402: f64, t12407: f64, t12409: f64, t12413: f64, t12429: f64, t1341: f64, t1343: f64, t3795: f64, t3803: f64, t3805: f64, t39936: f64, t39938: f64, t39945: f64, t820: f64) -> f64 {
    let t39948 = t39947 * t1354;
    let t39950 = t12365 * t3853;
    let t39952 = t12267 * t3789;
    let t39955 = t12267 * t3798;
    let t39956 = t39955 * t1354;
    let t39958 = t12385 * t12297;
    let t39960 = t12300 * t3858;
    let t39970 = t39936 - t1341 * t1343 * t820 * t39938 / 1024.0_f64 + 7.0_f64 / 192.0_f64 * t39945 - 119.0_f64 / 1152.0_f64 * t39948 - 119.0_f64 / 2304.0_f64 * t39950 + t39952 * t3795 / 256.0_f64 + 7.0_f64 / 384.0_f64 * t39956 - 7.0_f64 / 192.0_f64 * t39958 + 7.0_f64 / 384.0_f64 * t39960 + t12429 * t12409 / 64.0_f64 + t3803 * t3805 * t12402 * t12407 / 128.0_f64 - t12429 * t12413 / 256.0_f64;
    t39970
}
