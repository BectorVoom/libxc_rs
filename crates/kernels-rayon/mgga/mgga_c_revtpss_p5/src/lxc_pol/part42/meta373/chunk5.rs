//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1220/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1220(t6017: f64, t72: f64, t686: f64, t2798: f64, t5978: f64, t14568: f64, t4500: f64, t18699: f64, t231: f64, t2783: f64, t2782: f64, t18677: f64) -> (f64, f64, f64, f64, f64) {
    let t18725 = t6017 * t72;
    let t18726 = t18725 * t686;
    let t18727 = t2798 * t18726;
    let t18729 = t5978 * t72;
    let t18730 = t18729 * t686;
    let t18731 = t2798 * t18730;
    let t18733 = t14568 * t4500;
    let t18738 = t2783 * t18699 * t231;
    let t18739 = t2782 * t18738;
    let t18742 = t2783 * t18677 * t231;
    (t18727, t18731, t18733, t18739, t18742)
}
