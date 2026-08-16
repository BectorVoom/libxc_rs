//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2160/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2160(t27182: f64, t686: f64, t72: f64, t25387: f64, t2435: f64, t27334: f64, t10867: f64, t1949: f64, t14485: f64, t25399: f64, t27195: f64, t1955: f64, t27198: f64, t2769: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t99161 = t27182 * t72 * t686;
    let t99163 = 0.51405703062096148812e-1_f64 * t25387 * t99161;
    let t99166 = t2435 * t27334;
    let t99174 = t10867 * t1949;
    let t99186 = t25399 * t14485;
    let t99188 = t2435 * t27195;
    let t99191 = t1955 * t27198 * t2769;
    (t99161, t99163, t99166, t99174, t99186, t99188, t99191)
}
