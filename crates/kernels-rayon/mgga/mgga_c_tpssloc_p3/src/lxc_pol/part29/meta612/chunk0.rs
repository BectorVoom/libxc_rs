//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2051/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2051(t2109: f64, t83728: f64, t83737: f64, t24525: f64, t9239: f64, t39063: f64, t7245: f64, t2108: f64, t2240: f64, t2244: f64, t39049: f64, t9231: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t85473 = t2109 * t83728;
    let t85476 = t2109 * t83737;
    let t85480 = t9239 * t24525;
    let t85501 = t39063 * t7245;
    let t85507 = t2240 * t2244 * t2108;
    let t85510 = t39049 * t7245;
    let t85514 = t9231 * t24525;
    (t85473, t85476, t85480, t85501, t85507, t85510, t85514)
}
