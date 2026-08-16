//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1116/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1116(t154: f64, t19023: f64, t385: f64, t824: f64, t2185: f64, t6446: f64, t2380: f64, t6475: f64, t6479: f64, t3185: f64, t6412: f64, t3174: f64, t6423: f64, t68: f64) -> (f64, f64, f64, f64, f64) {
    let t19026 = t385 * t154 * t19023 * t824;
    let t19030 = t385 * t154 * t6446 * t2185;
    let t19033 = t2380 * t6475 * t6479;
    let t19036 = t3185 * t6475 * t6412;
    let t19039 = t3174 * t68 * t6423;
    (t19026, t19030, t19033, t19036, t19039)
}
