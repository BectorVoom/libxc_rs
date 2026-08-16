//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1143/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1143(t131: f64, t23121: f64, t9537: f64, t236: f64, t81613: f64, t23098: f64, t22822: f64, t281: f64, t6589: f64, t23124: f64, t23076: f64, t6597: f64) -> (f64, f64, f64) {
    let t81782 = t23121 * t131 * t9537;
    let t81783 = t81613 * t236;
    let t81785 = t81782 * t81783 * t23098;
    let t81788 = t22822 * t6589 * t281;
    let t81789 = t81788 * t23124;
    let t81792 = t6597 * t23076 * t281;
    (t81785, t81789, t81792)
}
