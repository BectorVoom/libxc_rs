//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2429/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2429(t20: f64, t2237: f64, t12: f64, t14: f64, t27: f64, t10285: f64, t596: f64, t10293: f64, t592: f64, t25: f64, t40649: f64, t10308: f64, t599: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t45941 = 840.0_f64 * t20 * t2237;
    let t45944 = 360.0_f64 * t12 * t14 * t27;
    let t45945 = t10285 * t596;
    let t45949 = t592 * t10293;
    let t45952 = 3024.0_f64 * t25 * t40649;
    let t45963 = t599 * t10308;
    (t45941, t45944, t45945, t45949, t45952, t45963)
}
