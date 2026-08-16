//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1143/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1143(t198: f64, t206: f64, t8656: f64, t11064: f64, t1032: f64, t7398: f64, t867: f64, t7060: f64, t7063: f64, t28425: f64, t8479: f64, t25386: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t121751 = t198 * t206 * t8656;
    let t121793 = t8656 * t11064;
    let t121803 = t7398 * t1032;
    let t121804 = t121803 * t867;
    let t121806 = t7063 * t121804 * t7060;
    let t121808 = t8479 * t28425;
    let t121809 = t25386 * t121808;
    (t121751, t121793, t121803, t121804, t121806, t121808, t121809)
}
