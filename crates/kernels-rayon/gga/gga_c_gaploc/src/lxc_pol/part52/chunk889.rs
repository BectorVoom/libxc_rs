//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 889/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk889(t45902: f64, t10914: f64, t2365: f64, t35446: f64, t13555: f64, t4614: f64, t833: f64, t10811: f64, t10978: f64, t3470: f64, t37061: f64, t36590: f64, t955: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t45903 = 0.14896037479937677779e-1_f64 * t45902;
    let t45905 = t10914 * t2365 * t35446;
    let t45906 = 0.89376224879626066674e-1_f64 * t45905;
    let t45913 = 0.15337170381568299871e2_f64 * t833 * t4614 * t13555;
    let t45915 = 0.85801175884441024006e1_f64 * t10811 * t10978;
    let t45922 = 0.10725146985555128001e1_f64 * t37061 * t3470;
    let t45931 = 0.23833659967900284446e0_f64 * t955 * t36590;
    (t45903, t45906, t45913, t45915, t45922, t45931)
}
