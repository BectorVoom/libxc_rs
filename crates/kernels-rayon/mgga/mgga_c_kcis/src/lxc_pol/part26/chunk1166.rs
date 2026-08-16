//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1166/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1166(t27636: f64, t7429: f64, t6176: f64, t1889: f64, t6207: f64, t6159: f64, t2256: f64, t23036: f64, t1650: f64, t2104: f64, t27584: f64, t4440: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t29568 = t27636 * t7429;
    let t29569 = t6176 * t29568;
    let t29574 = t6207 * t1889;
    let t29575 = t6159 * t29574;
    let t29578 = t23036 * t2256;
    let t29581 = t1650 * t2104;
    let t29582 = t27584 * t29581;
    let t29583 = t4440 * t29582;
    (t29568, t29569, t29574, t29575, t29578, t29582, t29583)
}
