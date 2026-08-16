//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 772/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk772(t6188: f64, t6461: f64, t6072: f64, t2168: f64, t2183: f64, t2191: f64, t2236: f64, t1632: f64, t2252: f64, t551: f64, t549: f64, t2097: f64, t547: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6462 = t6188 * t6461;
    let t6463 = t6462 * t6072;
    let t6465 = t2183 * t2168;
    let t6468 = t2236 * t2191;
    let t6470 = t1632 * t2252;
    let t6471 = t551 * t6470;
    let t6472 = t549 * t6471;
    let t6474 = t547 * t2097;
    (t6462, t6463, t6465, t6468, t6470, t6472, t6474)
}
