//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1103/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1103(t2107: f64, t33651: f64, t2014: f64, t2056: f64, t33602: f64, t34279: f64, t34285: f64, t34290: f64, t34294: f64, t34300: f64, t34304: f64, t34326: f64, t4248: f64, t569: f64, t651: f64, t6985: f64, t7359: f64, t7732: f64, t7746: f64, t7978: f64, t7988: f64, t8637: f64) -> (f64, f64) {
    let t34328 = t2107 * t33651;
    let t34329 = t2014 * t34328;
    let t34330 = -2.0_f64 * t2056 * t33602 - 2.0_f64 * t34279 * t651 - 2.0_f64 * t34290 * t651 + t34326 * t569 - 2.0_f64 * t4248 * t8637 - 2.0_f64 * t6985 * t7978 - 2.0_f64 * t6985 * t7988 - 2.0_f64 * t7359 * t7746 - 2.0_f64 * t7732 * t8637 - t34285 - t34294 + t34300 - t34304 - t34329;
    (t34328, t34330)
}
