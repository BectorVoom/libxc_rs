//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1115/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1115(t1962: f64, t41154: f64, t2411: f64, t605: f64, t2453: f64, t251: f64, t25304: f64, t7063: f64, t860: f64, t1113: f64, t555: f64, t1444: f64, t543: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t92742 = t1962 * t41154;
    let t92790 = t2411 * t605;
    let t93169 = t2453 * t251;
    let t93189 = t25304 * t251;
    let t93341 = t7063 * t860;
    let t94245 = t2411 * t1113;
    let t94382 = t2453 * t555;
    let t94390 = t25304 * t555;
    let t94396 = t543 * t1444;
    (t92742, t92790, t93169, t93189, t93341, t94245, t94382, t94390, t94396)
}
