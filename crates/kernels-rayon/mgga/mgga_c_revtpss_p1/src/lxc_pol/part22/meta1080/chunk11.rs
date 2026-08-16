//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3893/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3893(t22386: f64, t3915: f64, t686: f64, t72: f64, t49503: f64, t5722: f64, t213: f64, t22307: f64, t1358: f64, t2439: f64, t6888: f64, t785: f64) -> (f64, f64, f64, f64) {
    let t74794 = t3915 * t22386 * t72 * t686;
    let t74797 = t49503 * t5722;
    let t74802 = t213 * t22307;
    let t74807 = t2439 * t785 * t6888 * t1358;
    (t74794, t74797, t74802, t74807)
}
