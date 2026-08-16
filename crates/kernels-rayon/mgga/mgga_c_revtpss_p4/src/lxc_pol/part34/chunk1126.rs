//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1126/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1126(t27279: f64, t7058: f64, t72: f64, t7769: f64, t686: f64, t25375: f64, t25387: f64, t1955: f64, t7057: f64, t1949: f64, t2718: f64, t2411: f64, t7782: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t27338 = t7058 * t27279;
    let t27340 = t7769 * t72;
    let t27341 = t27340 * t686;
    let t27342 = t25375 * t27341;
    let t27344 = t25387 * t27341;
    let t27353 = t1955 * t7057;
    let t27357 = t2718 * t1949;
    let t27368 = t7782 * t2411;
    (t27338, t27340, t27341, t27342, t27344, t27353, t27357, t27368)
}
