//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 811/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk811(t21196: f64, t4334: f64, t1268: f64, t2923: f64, t4973: f64, t1091: f64, t5468: f64, t10864: f64, t5457: f64, t4969: f64, t21181: f64, t231: f64, t2918: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t21871 = t4334 * t21196;
    let t21875 = t2923 * t4973 * t1268;
    let t21877 = t1091 * t5468;
    let t21878 = t2923 * t21877;
    let t21881 = t10864 * t1091 * t5457;
    let t21885 = t2923 * t4969 * t1268;
    let t21893 = t231 * t2918 * t21181;
    (t21871, t21875, t21877, t21878, t21881, t21885, t21893)
}
