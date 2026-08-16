//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 641/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk641(t1003: f64, t1171: f64, t226: f64, t325: f64, t3807: f64, t120: f64, t860: f64, t108: f64, t124: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t24889 = t1003 * t1003;
    let t24890 = 1.0_f64 / t24889;
    let t24983 = t1171 * t1171;
    let t24985 = 1.0_f64 / t226 / t24983;
    let t25441 = t3807 * t325;
    let t25518 = t120 * t860;
    let t25525 = t108 * t124;
    (t24890, t24983, t24985, t25441, t25518, t25525)
}
