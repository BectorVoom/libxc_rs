//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 744/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk744(t10294: f64, t268: f64, t271: f64, t6546: f64, t154: f64, t3061: f64, t276: f64, t285: f64, t273: f64, t2928: f64, t941: f64, t2931: f64, t323: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10542 = 0.36793333333333333333e0_f64 * t10294;
    let t10544 = t268 * t6546 * t271;
    let t10545 = 0.93932222222222222223e0_f64 * t10544;
    let t10564 = t154 * t3061;
    let t10577 = 28.0_f64 / 27.0_f64 * t10544;
    let t10595 = 1.0_f64 / t276 / t285 / 4.0_f64;
    let t10599 = 1.0_f64/pow_3_2(t273);
    let t10608 = 0.28842592592592592592e-1_f64 * t10544;
    let t10629 = 1.0_f64 / t2928 / t941;
    let t10632 = 1.0_f64 / t2931 / t323;
    (t10542, t10544, t10545, t10564, t10577, t10595, t10599, t10608, t10629, t10632)
}
