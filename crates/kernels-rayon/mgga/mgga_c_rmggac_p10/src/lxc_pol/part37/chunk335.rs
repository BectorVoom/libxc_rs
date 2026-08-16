//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 335/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk335(t649: f64, t664: f64, t27: f64, t640: f64, t702: f64, t3066: f64, t3070: f64, t3082: f64, t36: f64, t699: f64, t305: f64, t326: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3166 = t649 * t664;
    let t3167 = t27 * t3166;
    let t3180 = t640 * t702;
    let t3184 = 0.14967802127329760705e-1_f64 * t3066;
    let t3185 = 0.10227998120342003148e-1_f64 * t3070;
    let t3187 = 0.68186654135613354325e-2_f64 * t3082;
    let t3188 = t699 * t36;
    let t3189 = t305 * t3188;
    let t3190 = 0.14967802127329760705e-1_f64 * t3189;
    let t3191 = t326 * t699;
    (t3167, t3180, t3184, t3185, t3187, t3188, t3190, t3191)
}
