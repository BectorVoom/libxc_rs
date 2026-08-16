//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 974/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk974(t222: f64, t30162: f64, t30170: f64, t44: f64, t291: f64, t2071: f64, t8459: f64, t294: f64, t30158: f64, t295: f64, t559: f64, t2231: f64, t7706: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t223 = t222 <= zeta_threshold;
    let t30172 = (t30162 + t30170) * t44;
    let t30173 = t30172 * t291;
    let t30174 = t2071 * t8459;
    let t30175 = t294 * t30174;
    let t30176 = 3.0_f64 / 16.0_f64 * t30175;
    let t30177 = piecewise3(t223, 0.0_f64, t30158);
    let t30178 = t295 * t30177;
    let t30179 = t30178 * t559;
    let t30180 = t294 * t30179;
    let t30181 = t30180 / 16.0_f64;
    let t30184 = t7706 * t2231;
    (t30173, t30176, t30181, t30184)
}
