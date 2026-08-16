//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2740/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2740(t12855: f64, t12916: f64, t20977: f64, t20913: f64, t3172: f64, t3711: f64, t21107: f64, t3704: f64, t17628: f64, t5373: f64, t20851: f64, t3678: f64) -> (f64, f64, f64, f64, f64) {
    let t71630 = t12855 * t12916 * t20977;
    let t71687 = t3711 * t3172 * t20913;
    let t71710 = t21107 * t3704;
    let t71718 = t5373 * t17628;
    let t71738 = t20851 * t3678;
    (t71630, t71687, t71710, t71718, t71738)
}
