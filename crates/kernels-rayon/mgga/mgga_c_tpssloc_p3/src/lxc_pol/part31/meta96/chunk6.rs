//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 596/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk596(t113: f64, t1983: f64, t2036: f64, t2040: f64, t2075: f64, t2079: f64, t2096: f64, t510: f64, t574: f64, t652: f64) -> f64 {
    let t2098 = -t113 * t2075 + t1983 * t2096 - t2036 * t510 - 2.0_f64 * t2040 * t652 + t2079 * t574;
    t2098
}
