//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 446/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk446(t8923: f64, t8955: f64, t8991: f64, t9028: f64, t495: f64, t570: f64, t515: f64, t498: f64, t5144: f64, t132: f64, t577: f64, t1392: f64, t202: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9030 = t8923 + t8955 + t8991 + t9028;
    let t9044 = t570 * t495;
    let t9045 = t515 * t9044;
    let t9049 = t570 * t498;
    let t9050 = t515 * t9049;
    let t9054 = t515 * t5144;
    let t9081 = t577 * t132;
    let t9085 = t1392 * t202;
    (t9030, t9045, t9050, t9054, t9081, t9085)
}
