//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 882/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk882(t13227: f64, t3564: f64, t3521: f64, t3551: f64, t12951: f64, t459: f64, t12830: f64, t3530: f64, t11313: f64, t1425: f64, t3555: f64, t3535: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13228 = t3564 * t13227;
    let t13231 = t3521 * t3551;
    let t13233 = t459 * t12951;
    let t13235 = t3530 * t13233 * t12830;
    let t13238 = t11313 * t1425;
    let t13240 = t3521 * t3555;
    let t13242 = t3521 * t3535;
    (t13228, t13231, t13235, t13238, t13240, t13242)
}
