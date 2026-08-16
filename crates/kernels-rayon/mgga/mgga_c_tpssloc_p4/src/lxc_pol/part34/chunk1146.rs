//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1146/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1146(t22893: f64, t23164: f64, t28345: f64, t28329: f64, t23185: f64, t28426: f64, t81914: f64, t28334: f64, t6547: f64, t28322: f64, t6579: f64, t1484: f64, t1519: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t98345 = t23164 * t22893 * t28345;
    let t98356 = t23164 * t22893 * t28329;
    let t98363 = t23185 * t81914 * t28426;
    let t98374 = t6547 * t28334;
    let t98380 = t6579 * t28322;
    let t98389 = t1519 * t1484;
    (t98345, t98356, t98363, t98374, t98380, t98389)
}
