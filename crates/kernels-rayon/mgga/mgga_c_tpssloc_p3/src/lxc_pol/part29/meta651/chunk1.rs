//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2176/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2176(t1983: f64, t7687: f64, t90065: f64, t26062: f64, t645: f64, t72: f64, t26066: f64, t2307: f64, t7431: f64, t1437: f64, t6509: f64, t1864: f64, t4021: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t90068 = 6.0_f64 * t1983 * t90065 * t7687;
    let t90072 = t72 * t26062 * t645;
    let t90076 = t72 * t26066 * t645;
    let t90080 = t72 * t7431 * t2307;
    let t90090 = t6509 * t1437;
    let t90094 = t1864 * t4021;
    (t90068, t90072, t90076, t90080, t90090, t90094)
}
