//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1068/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1068(t32715: f64, t7063: f64, t7286: f64, t32237: f64, t8477: f64) -> (f64, f64, f64) {
    let t32716 = t7063 * t32715;
    let t32718 = 0.25702851531048074406e-1_f64 * t32716 * t7286;
    let t32719 = t8477 * t32237;
    (t32716, t32718, t32719)
}
