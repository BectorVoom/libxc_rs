//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1184/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1184(t23741: f64, t3347: f64, t10215: f64, t599: f64, t475: f64, t2268: f64, t26938: f64, t6767: f64, t21389: f64, t7937: f64, t10178: f64, t6305: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31825 = 0.85365019907028448797e-1_f64 * t23741 * t3347;
    let t31828 = t599 * t10215;
    let t31829 = t31828 * t475;
    let t31835 = 0.68292015925622759036e0_f64 * t2268 * t26938 * t6767;
    let t31838 = 0.68292015925622759036e0_f64 * t2268 * t7937 * t21389;
    let t31840 = 0.34146007962811379518e0_f64 * t6305 * t10178;
    (t31825, t31828, t31829, t31835, t31838, t31840)
}
