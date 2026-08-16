//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1369/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1369(t18210: f64, t2237: f64, t29331: f64, t103301: f64, t1889: f64, t98239: f64, t1943: f64, t5426: f64, t833: f64, t98233: f64, t29404: f64, t7904: f64) -> (f64, f64, f64, f64) {
    let t103496 = t2237 * t18210 * t29331;
    let t103502 = t98239 * t1889 * t103301;
    let t103507 = t98233 * t5426 * t1943 * t833;
    let t103525 = t29404 * t7904;
    (t103496, t103502, t103507, t103525)
}
