//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 650/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk650(t1982: f64, t2314: f64, t7428: f64, t2191: f64, t2283: f64, t495: f64, t570: f64, t515: f64, t1971: f64, t7230: f64, t498: f64, t7231: f64) -> (f64, f64, f64, f64, f64) {
    let t9040 = t2314 * t7428 * t1982;
    let t9042 = t2191 * t2283;
    let t9044 = t570 * t495;
    let t9045 = t515 * t9044;
    let t9046 = t1971 * t9045;
    let t9047 = t7230 * t9046;
    let t9049 = t570 * t498;
    let t9050 = t515 * t9049;
    let t9051 = t7231 * t9050;
    (t9040, t9042, t9046, t9047, t9051)
}
