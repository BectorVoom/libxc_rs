//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 856/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk856(t3515: f64, t655: f64, t218: f64, t219: f64, t208: f64, t9161: f64, t5558: f64, t5560: f64, t7332: f64, t7465: f64, t7466: f64, t9178: f64, t9180: f64, t9185: f64, t9189: f64, t9192: f64) -> (f64, f64, f64, f64, f64) {
    let t9194 = t655 * t3515;
    let t9196 = t218 * t219 * t9194;
    let t9198 = t208 * t9161;
    let t9200 = t218 * t219 * t9198;
    let t9202 = 0.82524375e-1_f64 * t9178 + 0.16504875e0_f64 * t9180 - t5558 + 0.27595e0_f64 * t5560 + 0.5519e0_f64 * t7332 - t7465 - t7466 - 0.16557e0_f64 * t9185 + 0.49671e0_f64 * t9189 - 0.16557e0_f64 * t9192 + 0.248355e0_f64 * t9196 + 0.248355e0_f64 * t9200;
    (t9194, t9196, t9198, t9200, t9202)
}
