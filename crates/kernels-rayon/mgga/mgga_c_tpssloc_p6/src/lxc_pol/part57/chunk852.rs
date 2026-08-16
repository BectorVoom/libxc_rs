//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 852/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk852(t31170: f64, t32721: f64, t1831: f64, t8466: f64, t31137: f64, t7691: f64, t6888: f64, t7700: f64, t1985: f64, t1799: f64, t31193: f64, t6637: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t32722 = t31170 * t32721;
    let t32724 = t8466 * t1831;
    let t32731 = t31137 * t7691;
    let t32733 = 0.3289868133696452873e-1_f64 * t6888 * t32731;
    let t32735 = t31137 * t7700;
    let t32737 = 0.16449340668482264365e-1_f64 * t1985 * t32735;
    let t32740 = t31193 * t1799;
    let t32741 = t6637 * t32740;
    (t32722, t32724, t32731, t32733, t32735, t32737, t32740, t32741)
}
