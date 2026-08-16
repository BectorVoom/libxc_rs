//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 519/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk519(t2145: f64, t790: f64, t2112: f64, t2120: f64, t307: f64, t311: f64, t786: f64, t800: f64) -> (f64, f64) {
    let t2146 = t790 * t2145;
    let t2149 = 0.65854491829355115987e0_f64 * t2112 * t311 - 0.13170898365871023197e1_f64 * t786 * t800 + 0.13170898365871023197e1_f64 * t307 * t2120 - 0.65854491829355115987e0_f64 * t307 * t2146;
    (t2146, t2149)
}
