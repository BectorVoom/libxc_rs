//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 952/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk952(t1807: f64, t6434: f64, t12351: f64, t20356: f64, t820: f64, t1825: f64, t19956: f64, t5248: f64, t550: f64, t6330: f64, t12419: f64, t5249: f64) -> (f64, f64, f64, f64) {
    let t20420 = t1807 * t6434;
    let t20433 = t12351 * t820 * t20356;
    let t20442 = t5248 * t19956 * t1825;
    let t20448 = t550 * t6330;
    let t20450 = t12419 * t5249 * t20448;
    (t20420, t20433, t20442, t20450)
}
