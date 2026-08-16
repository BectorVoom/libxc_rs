//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1246/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1246(t18397: f64, t4645: f64, t4669: f64, t5527: f64, t4674: f64, t93: f64, t94: f64, t196: f64, t197: f64, t5322: f64, t30: f64, t4706: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21185 = t18397 * t4645;
    let t21187 = t5527 * t4669;
    let t21227 = t93 * t4674;
    let t21236 = t94 * t4674;
    let t21253 = t5322 * t196 * t197;
    let t21255 = t30 * t4706;
    (t21185, t21187, t21227, t21236, t21253, t21255)
}
