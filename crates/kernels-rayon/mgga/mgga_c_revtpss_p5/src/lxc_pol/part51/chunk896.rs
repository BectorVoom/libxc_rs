//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 896/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk896(t27142: f64, t28046: f64, t28171: f64, t28232: f64, t3: f64, t2042: f64, t5795: f64, t1916: f64, t7331: f64, t7334: f64, t1459: f64, t7950: f64, param_d: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t28234 = t27142 + t28046 + t28171 + t28232;
    let t28235 = t3 * t28234;
    let t28246 = param_d * t28234;
    let t28257 = 3.0_f64 * t5795 * t2042;
    let t28259 = 6.0_f64 * t1916 * t7331;
    let t28261 = 3.0_f64 * t1916 * t7334;
    let t28263 = 6.0_f64 * t1459 * t7950;
    (t28235, t28246, t28257, t28259, t28261, t28263)
}
