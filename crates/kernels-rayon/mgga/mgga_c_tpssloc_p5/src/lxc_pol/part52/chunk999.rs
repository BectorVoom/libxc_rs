//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 999/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk999(t218: f64, t25160: f64, t253: f64, t254: f64, t10109: f64, t1911: f64, t4272: f64, t25036: f64, t25042: f64, t25047: f64, t25049: f64, t25051: f64, t25056: f64, t25061: f64, t259: f64, t2597: f64, t4147: f64, t4301: f64, t6627: f64, t6632: f64, t6663: f64, t7538: f64) -> (f64, f64, f64, f64, f64) {
    let t25161 = t218 * t25160;
    let t25168 = t253 * t254;
    let t25169 = t10109 * t1911;
    let t25170 = t25169 * t4272;
    let t25173 = -0.41123351671205660912e-2_f64 * t25036 + 0.49348022005446793095e-1_f64 * t25042 + 0.16449340668482264365e-1_f64 * t25047 - 0.19190897446562641759e-1_f64 * t25049 + t25051 * t259 + 0.16449340668482264365e-1_f64 * t25056 + 0.82246703342411321825e-2_f64 * t25061 + t25161 * t259 - t2597 * t7538 - t6627 * t4301 + 2.0_f64 * t4147 * t6632 - t4147 * t6663 - 6.0_f64 * t25168 * t25170;
    (t25161, t25168, t25169, t25170, t25173)
}
