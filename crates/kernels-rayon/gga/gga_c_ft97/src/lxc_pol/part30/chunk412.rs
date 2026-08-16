//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 412/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk412(t263: f64, t6838: f64, t193: f64, t1173: f64, t1425: f64, t1091: f64, t6074: f64, t2599: f64, t1131: f64, t1424: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6839 = t6838 * t263;
    let t6840 = t193 * t6839;
    let t6843 = t1425 * t1173;
    let t6844 = t193 * t6843;
    let t6848 = t6074 * t1091;
    let t6849 = t2599 * t6848;
    let t6852 = t1424 * t1131;
    (t6839, t6840, t6843, t6844, t6848, t6849, t6852)
}
