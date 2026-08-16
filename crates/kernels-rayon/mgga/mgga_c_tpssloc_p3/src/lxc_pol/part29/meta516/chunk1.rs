//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1891/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1891(t28: f64, t870: f64, t4255: f64, t16596: f64, t23788: f64, t1081: f64, t1484: f64, t4119: f64, t25365: f64, t10143: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25891 = t870 * t28;
    let t25892 = t25891 * t4255;
    let t25898 = t23788 * t16596;
    let t25901 = t1081 * t1484;
    let t25905 = t28 * t4119;
    let t25921 = t23788 * t25365;
    let t25927 = t10143 * t28;
    (t25891, t25892, t25898, t25901, t25905, t25921, t25927)
}
