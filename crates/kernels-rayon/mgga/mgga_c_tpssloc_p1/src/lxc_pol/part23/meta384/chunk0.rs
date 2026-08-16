//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1187/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1187(t11677: f64, t15027: f64, t3624: f64, t52627: f64, t1213: f64, t1735: f64, t248: f64, t45017: f64, t10477: f64, t1742: f64, t11713: f64, t3503: f64) -> (f64, f64, f64, f64, f64) {
    let t52879 = t15027 * t11677;
    let t52903 = t3624 * t52627;
    let t53079 = t1213 * t248 * t45017 * t1735;
    let t53081 = t1742 * t10477;
    let t53083 = t11713 * t3503 * t53081;
    (t52879, t52903, t53079, t53081, t53083)
}
