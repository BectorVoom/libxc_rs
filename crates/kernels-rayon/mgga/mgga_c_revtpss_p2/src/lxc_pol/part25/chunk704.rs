//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 704/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk704(t198: f64, t532: f64, t539: f64, t73: f64, t241: f64, t4000: f64, t820: f64, t550: f64, t72: f64, t245: f64, t225: f64, t3999: f64) -> (f64, f64, f64, f64, f64) {
    let t5541 = t198 * t532;
    let t5650 = t539 * t73;
    let t5671 = t820 * t4000 * t241;
    let t5672 = t550 * t72;
    let t5673 = t5672 * t245;
    let t5744 = t225 * t3999;
    (t5541, t5650, t5671, t5673, t5744)
}
