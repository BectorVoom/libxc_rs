//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1154/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1154(t10041: f64, t6581: f64, t213: f64, t6589: f64, t9223: f64, t6593: f64, t23062: f64, t23066: f64, t1894: f64, t236: f64, t6591: f64, t9516: f64) -> (f64, f64, f64, f64) {
    let t81930 = t6581 * t10041;
    let t81933 = t9223 * t6589 * t213;
    let t81934 = t81933 * t6593;
    let t81936 = t23062 * t23066;
    let t81940 = t6591 * t1894 * t236 * t9516;
    (t81930, t81934, t81936, t81940)
}
