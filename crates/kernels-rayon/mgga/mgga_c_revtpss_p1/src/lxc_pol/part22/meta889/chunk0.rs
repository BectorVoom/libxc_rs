//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3077/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3077(t11409: f64, t1621: f64, t2968: f64, t300: f64, t3012: f64, t11507: f64, t15494: f64, t11223: f64, t379: f64, t4930: f64, t989: f64, t11199: f64, t1646: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t52837 = t11409 * t1621;
    let t52840 = t2968 * t1621;
    let t52877 = t300 * t3012;
    let t52894 = t300 * t11507;
    let t52921 = t300 * t15494;
    let t52927 = t11223 * t379;
    let t52994 = t989 * t4930;
    let t53014 = t1646 * t11199;
    (t52837, t52840, t52877, t52894, t52921, t52927, t52994, t53014)
}
