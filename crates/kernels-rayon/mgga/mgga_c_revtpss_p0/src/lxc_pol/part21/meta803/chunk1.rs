//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2918/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2918(t15494: f64, t300: f64, t983: f64, t52516: f64, t52647: f64, t52650: f64, t52652: f64, t52910: f64, t52912: f64, t52914: f64, t52916: f64, t52918: f64, t52920: f64) -> (f64, f64) {
    let t52921 = t300 * t15494;
    let t52923 = 0.17544670867903938621e1_f64 * t52921 * t983;
    let t52924 = -t52910 - t52516 - t52912 + t52914 - t52916 - t52918 - t52920 - t52923 + t52647 + t52650 + t52652;
    (t52923, t52924)
}
