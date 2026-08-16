//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3041/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3041(t10542: f64, t14563: f64, t14519: f64, t2470: f64, t2798: f64, t231: f64, t51049: f64, t2782: f64, t2797: f64, t14663: f64, t686: f64, t72: f64) -> (f64, f64, f64, f64, f64) {
    let t51429 = t10542 * t14563;
    let t51434 = t2798 * t14519 * t2470;
    let t51436 = t51049 * t231;
    let t51438 = t2782 * t2797 * t51436;
    let t51442 = t2798 * t14663 * t72 * t686;
    (t51429, t51434, t51436, t51438, t51442)
}
