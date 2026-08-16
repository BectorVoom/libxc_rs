//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1079/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1079(t121134: f64, t121365: f64, t32296: f64, t531: f64, t2045: f64, t7318: f64, t2037: f64, t7337: f64, t1455: f64, t8617: f64, t32378: f64, t571: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t121366 = t121365 * t121134;
    let t121441 = t531 * t32296;
    let t121458 = t7318 * t2045;
    let t121460 = t2037 * t7337;
    let t121468 = t1455 * t8617;
    let t121470 = t571 * t32378;
    (t121366, t121441, t121458, t121460, t121468, t121470)
}
