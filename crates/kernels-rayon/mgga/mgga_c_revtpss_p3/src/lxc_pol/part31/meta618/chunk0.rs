//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2065/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2065(t25375: f64, t99125: f64, t25387: f64, t27182: f64, t686: f64, t72: f64, t2435: f64, t27334: f64, t10867: f64, t1949: f64, t14485: f64, t25399: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t99127 = 0.28912093960683998208e-1_f64 * t25375 * t99125;
    let t99147 = 0.51405703062096148812e-1_f64 * t25387 * t99125;
    let t99161 = t27182 * t72 * t686;
    let t99163 = 0.51405703062096148812e-1_f64 * t25387 * t99161;
    let t99166 = t2435 * t27334;
    let t99174 = t10867 * t1949;
    let t99186 = t25399 * t14485;
    (t99127, t99147, t99161, t99163, t99166, t99174, t99186)
}
