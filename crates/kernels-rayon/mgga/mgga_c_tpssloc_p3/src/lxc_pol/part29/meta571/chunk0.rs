//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1988/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1988(t1834: f64, t3850: f64, t16463: f64, t225: f64, t16448: f64, t12020: f64, t1842: f64, t16468: f64, t16458: f64, t16486: f64, t3701: f64, t112: f64, t16506: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t54918 = t1834 * t3850;
    let t55069 = t16463 * t225;
    let t55093 = t16448 * t225;
    let t55118 = t12020 * t1842;
    let t55134 = t16468 * t225;
    let t55150 = t16458 * t225;
    let t55169 = t16486 * t3701;
    let t55341 = t16506 * t112;
    (t54918, t55069, t55093, t55118, t55134, t55150, t55169, t55341)
}
