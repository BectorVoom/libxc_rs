//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2769/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2769(t14322: f64, t2626: f64, t10326: f64, t4401: f64, t4402: f64, t4398: f64, t9425: f64, t10555: f64, t14613: f64, t10565: f64, t1532: f64, t9419: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t50883 = t14322 * t2626;
    let t50884 = 0.35089341735807877242e1_f64 * t50883;
    let t50887 = 12.0_f64 * t4401 * t4402 * t10326;
    let t50888 = t4398 * t9425;
    let t50889 = 0.35089341735807877242e1_f64 * t50888;
    let t50890 = t14613 * t10555;
    let t50891 = 36.0_f64 * t50890;
    let t50892 = t1532 * t10565;
    let t50893 = t4398 * t9419;
    (t50884, t50887, t50889, t50891, t50892, t50893)
}
