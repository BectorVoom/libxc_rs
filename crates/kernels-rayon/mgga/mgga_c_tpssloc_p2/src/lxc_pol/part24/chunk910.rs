//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 910/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk910(t10266: f64, t10357: f64, t225: f64, t68: f64, t369: f64, t10195: f64, t2979: f64, t1031: f64, t3077: f64, t1036: f64, t3078: f64, t1032: f64, t3082: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10358 = t10266 + t10357;
    let t10359 = t10358 * t225;
    let t10360 = t10359 * t68;
    let t10361 = t10360 * t369;
    let t10364 = t2979 * t10195;
    let t10367 = t3077 * t1031;
    let t10370 = t3078 * t1036;
    let t10372 = t1032 * t3082;
    (t10358, t10359, t10360, t10361, t10364, t10367, t10370, t10372)
}
