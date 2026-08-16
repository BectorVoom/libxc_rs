//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 615/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk615(t2568: f64, t27924: f64, t681: f64, t6839: f64, t1168: f64, t24429: f64, t1173: f64, t6062: f64, t193: f64, t6838: f64, t771: f64, t1425: f64, t4003: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27925 = t2568 * t27924;
    let t27929 = t681 * t6839;
    let t27934 = t24429 * t1168;
    let t27938 = t6062 * t1173;
    let t27939 = t193 * t27938;
    let t27942 = t6838 * t771;
    let t27943 = t193 * t27942;
    let t27946 = t1425 * t4003;
    (t27925, t27929, t27934, t27939, t27943, t27946)
}
