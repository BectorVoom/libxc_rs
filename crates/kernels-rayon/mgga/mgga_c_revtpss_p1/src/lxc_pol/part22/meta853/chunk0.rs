//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2995/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2995(t14426: f64, t72: f64, t757: f64, t14616: f64, t2619: f64, t14386: f64, t2615: f64, t198: f64, t775: f64, t10565: f64, t1469: f64, t706: f64) -> (f64, f64, f64, f64, f64) {
    let t49986 = t14426 * t72 * t757;
    let t50047 = t14616 * t2619;
    let t50058 = t14386 * t2615;
    let t50080 = t198 * t775;
    let t50084 = t706 * t10565 * t1469;
    (t49986, t50047, t50058, t50080, t50084)
}
