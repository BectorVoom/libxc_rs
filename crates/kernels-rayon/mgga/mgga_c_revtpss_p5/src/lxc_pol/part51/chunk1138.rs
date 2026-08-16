//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1138/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1138(t126121: f64, t839: f64, t33707: f64, t686: f64, t72: f64, t32469: f64, t33698: f64, t119982: f64, t119837: f64, t14686: f64, t1559: f64, t120011: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t126122 = t126121 * t839;
    let t126125 = t33707 * t72 * t686;
    let t126126 = t32469 * t126125;
    let t126129 = t33698 * t72 * t686;
    let t126130 = t119982 * t126129;
    let t126133 = t14686 * t119837 * t1559;
    let t126134 = t120011 * t126133;
    (t126122, t126125, t126126, t126129, t126130, t126133, t126134)
}
