//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 825/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk825(t2236: f64, t2727: f64, t2219: f64, t2670: f64, t2177: f64, t2699: f64, t2526: f64, t788: f64, t2207: f64, t785: f64, t2841: f64, t481: f64) -> (f64, f64, f64, f64, f64) {
    let t7397 = 0.23115257973478049502e0_f64 * t2236 * t2727;
    let t7399 = 0.69345773920434148506e0_f64 * t2670 * t2219;
    let t7401 = 0.25610080155860322884e0_f64 * t2177 * t2699;
    let t7402 = t788 * t2526;
    let t7405 = 0.34930954652346593434e-1_f64 * t2207 * t785 * t7402;
    let t7406 = t2841 * t481;
    (t7397, t7399, t7401, t7405, t7406)
}
