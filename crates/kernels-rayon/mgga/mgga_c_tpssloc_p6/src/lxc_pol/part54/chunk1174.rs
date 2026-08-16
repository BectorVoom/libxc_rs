//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1174/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1174(t31160: f64, t31177: f64, t31157: f64, t31163: f64, t31166: f64, t31173: f64, t31179: f64, t31576: f64, t539: f64, t225: f64, t567: f64, t7191: f64) -> (f64, f64, f64, f64, f64) {
    let t31578 = 0.26915170729426927235e-3_f64 * t31160;
    let t31582 = 7.0_f64 / 1152.0_f64 * t31177;
    let t31584 = -t31576 - 0.96894614625936938046e-2_f64 * t31157 - t31578 - 0.16149102437656156341e-2_f64 * t31163 + t31166 / 768.0_f64 - t31173 / 768.0_f64 - t31582 - t31179 / 192.0_f64;
    let t31585 = t539 * t31584;
    let t31589 = t7191 * t225 * t567;
    (t31578, t31582, t31584, t31585, t31589)
}
