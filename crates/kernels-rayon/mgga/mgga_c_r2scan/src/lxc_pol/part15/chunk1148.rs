//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1148/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1148(t39770: f64, t37890: f64, t924: f64, t24031: f64, t3332: f64, t7628: f64, t24035: f64, t6165: f64, t11646: f64, t22731: f64, t11649: f64, t25169: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39771 = 0.25610080155860322884e0_f64 * t39770;
    let t39772 = t37890 * t924;
    let t39775 = t7628 * t3332 * t24031;
    let t39778 = t6165 * t3332 * t24035;
    let t39780 = t22731 * t11646;
    let t39782 = t25169 * t11649;
    (t39771, t39772, t39775, t39778, t39780, t39782)
}
