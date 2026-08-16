//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 930/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk930(t3590: f64, t72: f64, t732: f64, t2222: f64, t3560: f64, t1289: f64, t724: f64, t581: f64, t3564: f64, t3589: f64, t680: f64, t2345: f64, t3557: f64) -> (f64, f64, f64, f64, f64) {
    let t10684 = t3590 * t72;
    let t10686 = 0.36622894612013090108e-3_f64 * t10684 * t732;
    let t10687 = t3560 * t2222;
    let t10689 = t724 * t1289;
    let t10690 = t10689 * t581;
    let t10692 = 24.0_f64 * t3564 * t10690;
    let t10698 = t680 * t3589;
    let t10701 = t3557 * t2345;
    (t10686, t10687, t10692, t10698, t10701)
}
