//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 746/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk746(t2610: f64, t6581: f64, t2230: f64, t6589: f64, t213: f64, t6593: f64, t1894: f64, t236: f64, t2553: f64, t6591: f64, t229: f64, t6546: f64) -> (f64, f64, f64, f64, f64) {
    let t23059 = t6581 * t2610;
    let t23061 = t2230 * t6589;
    let t23062 = t23061 * t213;
    let t23063 = t23062 * t6593;
    let t23066 = t1894 * t236 * t2553;
    let t23067 = t6591 * t23066;
    let t23069 = t6546 * t229;
    (t23059, t23062, t23063, t23067, t23069)
}
