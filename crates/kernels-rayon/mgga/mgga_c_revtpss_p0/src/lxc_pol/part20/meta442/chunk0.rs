//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1687/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1687(t12: f64, t14: f64, t27: f64, t10285: f64, t596: f64, t2231: f64, t2237: f64, t10293: f64, t592: f64, t25: f64, t40649: f64, t45927: f64, t45929: f64, t45931: f64, t45933: f64, t45935: f64, t45937: f64, t45939: f64, t45941: f64) -> f64 {
    let t45944 = 360.0_f64 * t12 * t14 * t27;
    let t45945 = t10285 * t596;
    let t45946 = 2880.0_f64 * t45945;
    let t45947 = t2231 * t2237;
    let t45948 = 7560.0_f64 * t45947;
    let t45949 = t592 * t10293;
    let t45950 = 8064.0_f64 * t45949;
    let t45952 = 3024.0_f64 * t25 * t40649;
    let t45953 = t45927 - t45929 + t45931 + t45933 - t45935 + t45937 - t45939 + t45941 + t45944 - t45946 + t45948 - t45950 + t45952;
    t45953
}
