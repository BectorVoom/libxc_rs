//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 736/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk736<F: Float>(t185: F, t2494: F, t9367: F, t9368: F, t1340: F, t9283: F, t9286: F, t9289: F, t9292: F, t9296: F, t9298: F, t9300: F, t9303: F, t738: F, t745: F, t1320: F, t3853: F) -> (F, F, F, F, F, F, F) {
    let t9371 = 1.0 / t2494 / t185;
    let t9372 = t9367 * t9368 * t9371;
    let t9374 = 0.10254018858216406658e4 * t1340 * t9372;
    let t9385 = -0.34523333333333333333e1 * t9283 + 0.23015555555555555556e1 * t9286 - 0.26851481481481481482e1 * t9289 - 0.93932222222222222223e0 * t9292 + 0.73355e-1 * t9296 - 0.14671e0 * t9298 - 0.17116166666666666667e0 * t9300 - 0.36793333333333333333e0 * t9303;
    let t9387 = t738 * t9385 * t745;
    let t9389 = 0.5848223622634646207e0 * t1340 * t9387;
    let t9391 = 12.0 * t1320 * t3853;
    (t9371, t9372, t9374, t9385, t9387, t9389, t9391)
}
