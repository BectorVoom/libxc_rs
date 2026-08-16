//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1083/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1083(t240: f64, t31197: f64, t31199: f64, t31201: f64, t31203: f64, t31206: f64, t31209: f64, t31406: f64, t31798: f64, t297: f64, t294: f64, t2351: f64, t7728: f64) -> (f64, f64) {
    let t31800 = t240 * t31798 + t31197 - t31199 + t31201 - t31203 - t31206 + t31209 - t31406;
    let t31801 = t297 * t31800;
    let t31802 = t294 * t31801;
    let t31803 = t31802 / 16.0_f64;
    let t31804 = t7728 * t2351;
    (t31803, t31804)
}
