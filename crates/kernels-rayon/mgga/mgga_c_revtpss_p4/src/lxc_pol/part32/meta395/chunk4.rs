//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1371/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1371(t124: f64, t18392: f64, t800: f64, t828: f64, t855: f64, t221: f64, t2675: f64, t5962: f64, t2674: f64, t10756: f64, t10758: f64, t10762: f64, t14836: f64, t14837: f64, t14839: f64, t14846: f64, t14850: f64, t14859: f64, t14864: f64, t799: f64, t851: f64) -> (f64, f64, f64, f64) {
    let t18393 = t124 * t18392;
    let t18394 = t800 * t18393;
    let t18398 = t855 * t828 * t18392;
    let t18402 = t2675 * t221 * t5962;
    let t18403 = t2674 * t18402;
    let t18405 = -t14836 + 0.80031500487063509015e-2_f64 * t14837 + 0.10841600599314203355e-2_f64 * t14839 - t10756 - t10758 - 0.60976381323476959249e-3_f64 * t14846 - 0.45178982497454656791e-5_f64 * t10762 - 0.15244095330869239812e-3_f64 * t14850 - t14859 + t14864 - t799 * t18394 / 48.0_f64 - 0.85748036236139473944e-3_f64 * t851 * t18398 - 0.50820002809285328225e-4_f64 * t18403;
    (t18394, t18398, t18402, t18405)
}
