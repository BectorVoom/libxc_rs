//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2248/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2248(t17152: f64, t42972: f64, t973: f64, t10876: f64, t13969: f64, t17983: f64, t13995: f64, t14501: f64, t10422: f64, t18020: f64, t3070: f64, t10883: f64, t17979: f64) -> (f64, f64, f64, f64, f64) {
    let t62766 = t973 * t42972 * t17152;
    let t62778 = t10876 * t13969 * t17983;
    let t62780 = t13995 * t14501;
    let t62811 = t3070 * t10422 * t18020;
    let t62816 = t10883 * t13969 * t17979;
    (t62766, t62778, t62780, t62811, t62816)
}
