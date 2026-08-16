//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1237/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1237(t169: f64, t2084: f64, t2089: f64, t32163: f64, t32514: f64, t7634: f64, t8942: f64, t10763: f64, t7137: f64, t21571: f64, t3440: f64, t10760: f64, t7129: f64) -> (f64, f64, f64, f64, f64) {
    let t32517 = 0.92286314761706691402e-1_f64 * t32163 * t2084 * t169 * t2089 * t32514;
    let t32520 = 0.15381052460284448567e-1_f64 * t32163 * t7634 * t8942;
    let t32522 = 0.6152420984113779427e-1_f64 * t7137 * t10763;
    let t32524 = 0.23071578690426672851e-1_f64 * t21571 * t3440;
    let t32526 = 0.46143157380853345702e-1_f64 * t7129 * t10760;
    (t32517, t32520, t32522, t32524, t32526)
}
