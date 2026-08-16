//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2266/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2266(t30923: f64, t3801: f64, t105665: f64, t105669: f64, t111864: f64, t111913: f64, t111959: f64, t112009: f64, t112051: f64, t112092: f64, t112138: f64, t112564: f64, t112602: f64, t112645: f64, t112697: f64, t112744: f64, t112787: f64, t112846: f64, t112899: f64, t112950: f64, t1298: f64, t1300: f64, t1832: f64, t198: f64, t21635: f64, t27037: f64, t27041: f64, t29317: f64, t29322: f64, t336: f64, t5023: f64, t5501: f64, t6748: f64, t6752: f64, t7673: f64, t97491: f64, t97498: f64) -> f64 {
    let t112958 = t30923 * t3801;
    let t112989 = t198 * t336 * (t111864 + t111913 + t111959 + t112009 + t112051 + t112092 + t112138 + t112564 + t112602 + t112645 + t112697 + t112744 + t112787 + t112846 + t112899 + t112950) * t1300 - t5023 * t112958 * t1298 - 2.0_f64 * t5023 * t105665 * t1832 + 4.0_f64 * t5023 * t105669 * t29322 - 2.0_f64 * t5023 * t29317 * t5501 + 2.0_f64 * t5023 * t97491 * t6752 - 6.0_f64 * t5023 * t97498 * t6752 * t1298 + 4.0_f64 * t5023 * t27041 * t1832 * t5501 - t5023 * t27037 * t6748 + 2.0_f64 * t5023 * t27041 * t6748 * t1298 - t5023 * t7673 * t21635;
    t112989
}
