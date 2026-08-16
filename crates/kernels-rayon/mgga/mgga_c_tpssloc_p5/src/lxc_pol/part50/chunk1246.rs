//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1246/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1246(t114418: f64, t1983: f64, t7687: f64, t24994: f64, t8449: f64, t24996: f64, t15868: f64, t8489: f64, t31246: f64, t7754: f64, t22751: f64, t32731: f64) -> (f64, f64, f64, f64, f64) {
    let t120171 = 3.0_f64 * t1983 * t114418 * t7687;
    let t120172 = t8449 * t24994;
    let t120173 = t120172 * t24996;
    let t120176 = t1983 * t8489 * t15868;
    let t120177 = t31246 * t7754;
    let t120179 = t22751 * t32731;
    (t120171, t120173, t120176, t120177, t120179)
}
