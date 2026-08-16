//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1818/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1818(t30: f64, t48292: f64, t48294: f64, t85929: f64, t85931: f64, t21906: f64, t22670: f64, t3833: f64, t47025: f64, t513: f64, t5549: f64, t5824: f64, t87125: f64, t91797: f64, t91802: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t91982 = 960.0_f64 * t48292;
    let t91983 = 480.0_f64 * t48294;
    let t91984 = 16.0_f64 * t85929;
    let t91985 = 16.0_f64 * t85931;
    let t91997 = piecewise3(t31, 0.0_f64, 40.0_f64 / 81.0_f64 * t47025 * t91797 - 16.0_f64 / 9.0_f64 * t21906 * t5824 + 4.0_f64 / 3.0_f64 * t3833 * t91802 + 16.0_f64 / 9.0_f64 * t5549 * t22670 + 4.0_f64 / 3.0_f64 * t513 * t87125);
    (t91982, t91983, t91984, t91985, t91997)
}
