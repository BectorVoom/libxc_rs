//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1403/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1403(t114601: f64, t1527: f64, t1888: f64, t23270: f64, t118892: f64, t118894: f64, t118901: f64, t118904: f64, t121607: f64, t13065: f64, t13463: f64, t2054: f64, t218: f64, t25188: f64, t25200: f64, t259: f64, t2713: f64, t2718: f64, t31416: f64, t33452: f64, t6662: f64, t7087: f64, t7092: f64, t7841: f64, t855: f64, t8553: f64, t8563: f64, t87758: f64, t98975: f64) -> f64 {
    let t121689 = t1888 * t23270 * t114601 * t1527;
    let t121691 = t218 * t121607 * t259 + t118892 - t118894 + 2.0_f64 * t25188 * t7092 - t87758 * t2054 - t13463 * t8563 + 2.0_f64 * t7087 * t25200 + 2.0_f64 * t13065 * t8553 + 2.0_f64 * t855 * t2718 * t7841 * t6662 + 2.0_f64 * t2713 * t33452 - 6.0_f64 * t98975 * t31416 - t118901 + t118904 + 0.16449340668482264365e-1_f64 * t121689;
    t121691
}
