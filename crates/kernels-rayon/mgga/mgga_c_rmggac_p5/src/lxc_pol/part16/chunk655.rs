//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 655/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk655(t2466: f64, t4041: f64, t2435: f64, t4965: f64, t2262: f64, t623: f64, t570: f64, t8264: f64, t1356: f64, t1668: f64, t2265: f64, t2228: f64, t551: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9410 = t4041 * t2466;
    let t9423 = t4965 * t2435;
    let t9425 = t623 * t2262;
    let t9427 = t8264 * t570;
    let t9428 = t1356 * t9427;
    let t9435 = t1668 * t2265;
    let t9437 = t2228 * t551;
    (t9410, t9423, t9425, t9427, t9428, t9435, t9437)
}
