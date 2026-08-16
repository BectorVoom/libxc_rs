//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 764/1310 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk764(t172: f64, t200: f64, t6: f64, t103: f64, t8948: f64, t4048: f64, t667: f64, t4043: f64, t169: f64, t134: f64, t674: f64, t1662: f64, t3031: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t8950 = t6 * t200 * t172;
    let t8951 = t103 * t8950;
    let t8952 = t8948 * t8951;
    let t8954 = t4048 * t667;
    let t8955 = t8954 * t4043;
    let t8956 = t169 * t8955;
    let t8957 = pi * t6;
    let t8958 = t134 * t674;
    let t8959 = t8958 * t172;
    let t8960 = t8957 * t8959;
    let t8961 = t8956 * t8960;
    let t8963 = t1662 * t3031;
    (t8950, t8951, t8952, t8957, t8958, t8959, t8960, t8961, t8963)
}
