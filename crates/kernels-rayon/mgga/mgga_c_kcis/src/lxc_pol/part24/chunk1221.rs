//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1221/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1221(t1021: f64, t19862: f64, t19879: f64, t7754: f64, t26891: f64, t6693: f64, t5026: f64, t5068: f64, t6717: f64, t9532: f64, t19870: f64, t7748: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t99931 = t1021 * t19862;
    let t99933 = t7754 * t19879;
    let t99935 = t26891 * t6693;
    let t99937 = t5026 * t5068;
    let t99939 = t9532 * t6717;
    let t99941 = t7748 * t19870;
    (t99931, t99933, t99935, t99937, t99939, t99941)
}
