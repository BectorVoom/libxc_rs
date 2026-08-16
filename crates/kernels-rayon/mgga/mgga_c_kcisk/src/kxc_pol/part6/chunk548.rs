//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 548/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk548(t222: f64, t227: f64, t229: f64, t3289: f64, t7715: f64, t7718: f64, t44: f64, t7714: f64, t291: f64, t7710: f64, t295: f64, t559: f64, t294: f64, t2071: f64, t2351: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t223 = t222 <= zeta_threshold;
    let t228 = t227 <= zeta_threshold;
    let t7722 = piecewise3(t228, 0.0_f64, 4.0_f64 / 9.0_f64 * t3289 * t7715 + 4.0_f64 / 3.0_f64 * t229 * t7718);
    let t7724 = (t7714 + t7722) * t44;
    let t7725 = t7724 * t291;
    let t7727 = piecewise3(t223, 0.0_f64, t7710);
    let t7728 = t295 * t7727;
    let t7729 = t7728 * t559;
    let t7730 = t294 * t7729;
    let t7732 = t2071 * t2351;
    (t7724, t7725, t7728, t7730, t7732)
}
