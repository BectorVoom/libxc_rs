//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1088/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1088(t4533: f64, t72: f64, t686: f64, t2465: f64, t1569: f64, t867: f64, t786: f64, t2467: f64, t122: f64, t4480: f64, t2466: f64, t10995: f64) -> (f64, f64, f64, f64) {
    let t14982 = t4533 * t72;
    let t14983 = t14982 * t686;
    let t14985 = 0.19514881078765566038e-1_f64 * t2465 * t14983;
    let t14986 = t1569 * t867;
    let t14987 = t786 * t14986;
    let t14989 = 0.19514881078765566038e-1_f64 * t14987 * t2467;
    let t14990 = t4480 * t122;
    let t14991 = t14990 * t2466;
    let t14992 = t10995 * t14991;
    (t14985, t14987, t14989, t14992)
}
