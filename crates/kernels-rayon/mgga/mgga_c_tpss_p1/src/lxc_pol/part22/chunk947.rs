//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 947/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk947(t3090: f64, t774: f64, t3069: f64, t3067: f64, t3138: f64, t9555: f64, t294: f64, t2966: f64, t458: f64, t8556: f64, t1108: f64, t8550: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9561 = t774 * t3090;
    let t9562 = t9561 * t3069;
    let t9563 = t3067 * t9562;
    let t9573 = t3138 * t9555;
    let t9589 = t294 * t2966;
    let t9605 = t458 * t8556;
    let t9607 = t8550 * t1108 * t9605;
    (t9561, t9563, t9573, t9589, t9605, t9607)
}
