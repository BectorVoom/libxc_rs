//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2085/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2085(t14540: f64, t14572: f64, t14953: f64, t14976: f64, t868: f64, t4533: f64, t72: f64, t686: f64, t2465: f64, t1569: f64, t867: f64, t786: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14978 = t14540 + t14572 + t14953 + t14976;
    let t14979 = t868 * t14978;
    let t14982 = t4533 * t72;
    let t14983 = t14982 * t686;
    let t14985 = 0.19514881078765566038e-1_f64 * t2465 * t14983;
    let t14986 = t1569 * t867;
    let t14987 = t786 * t14986;
    (t14978, t14979, t14982, t14983, t14985, t14986, t14987)
}
