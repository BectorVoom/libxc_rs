//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 740/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk740(t4210: f64, t7932: f64, t7942: f64, t609: f64, t862: f64, t865: f64, t2124: f64, t310: f64, t611: f64, t848: f64, t315: f64, t7941: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7943 = t7932 * t4210;
    let t7944 = t7942 * t7943;
    let t7948 = t862 * t609;
    let t7950 = 0.13170898365871023197e1_f64 * t7948 * t865;
    let t7957 = t310 * t2124;
    let t7962 = 0.65854491829355115987e0_f64 * t848 * t611;
    let t7963 = t315 * t7941;
    (t7943, t7944, t7948, t7950, t7957, t7962, t7963)
}
