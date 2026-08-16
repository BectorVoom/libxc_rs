//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1611/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1611(t1134: f64, t20356: f64, t5071: f64, t5079: f64, t3390: f64, t6449: f64, t12331: f64, t6442: f64, t5087: f64, t3407: f64, t1139: f64, t20337: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t20357 = t20356 * t1134;
    let t20359 = t5071 * t5079;
    let t20361 = t3390 * t6449;
    let t20362 = t20361 * t1134;
    let t20365 = t12331 * t6442;
    let t20366 = t20365 * t1134;
    let t20368 = t5087 * t5079;
    let t20370 = t3407 * t6449;
    let t20371 = t20370 * t1134;
    let t20373 = t1139 * t20337;
    (t20357, t20359, t20362, t20366, t20368, t20371, t20373)
}
