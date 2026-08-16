//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 133/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk133(t469: f64, t470: f64, t468: f64, t415: f64, t338: f64, t412: f64, t196: f64) -> (f64, f64, f64, f64, f64) {
    let t471 = t469 * t470;
    let t472 = t468 * t471;
    let t473 = t415 * t472;
    let t475 = t338 * t412 + 0.24872916666666666666e-2_f64 * t473;
    let t476 = t338 * t196;
    (t471, t472, t473, t475, t476)
}
