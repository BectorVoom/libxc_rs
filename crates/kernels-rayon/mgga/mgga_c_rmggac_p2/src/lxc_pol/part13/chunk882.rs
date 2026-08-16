//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 882/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk882(t2868: f64, t7578: f64, t623: f64, t7191: f64, t7194: f64, t2283: f64, t35384: f64, t2286: f64, t1175: f64, t1971: f64, t511: f64, t558: f64, t8517: f64) -> (f64, f64, f64, f64, f64) {
    let t39568 = t2868 * t7578;
    let t39570 = t623 * t7191;
    let t39571 = t39570 * t7194;
    let t39577 = t35384 * t2283;
    let t39584 = t35384 * t2286;
    let t39589 = t8517 * t1971 * t511 * t558 * t1175;
    (t39568, t39571, t39577, t39584, t39589)
}
