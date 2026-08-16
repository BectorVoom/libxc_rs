//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1121/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1121(t4166: f64, t6613: f64, t1878: f64, t23033: f64, t253: f64, t254: f64, t10109: f64, t1911: f64, t225: f64, t7492: f64, t1484: f64, t857: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25146 = t4166 * t6613;
    let t25154 = t1878 * t23033;
    let t25168 = t253 * t254;
    let t25169 = t10109 * t1911;
    let t25188 = t7492 * t225;
    let t25191 = t857 * t1484;
    (t25146, t25154, t25168, t25169, t25188, t25191)
}
