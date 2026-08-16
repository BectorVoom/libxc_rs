//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1103/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1103(t11848: f64, t11850: f64, t11853: f64, t11896: f64, t11899: f64, t11904: f64, t11908: f64, t11913: f64, t11916: f64, t11919: f64, t11922: f64, t11925: f64, t12093: f64, t12109: f64, t12115: f64, t12133: f64, t9182: f64, t9183: f64, t9192: f64, t9194: f64, t9196: f64, t9214: f64) -> f64 {
    let t12135 = -t9182 + 0.18396666666666666667e-1_f64 * t9183 + 0.18396666666666666667e0_f64 * t9192 - 0.5519e-1_f64 * t9194 - 0.11038e0_f64 * t9196 - t12093 + 0.82785e-1_f64 * t11848 + 0.91983333333333333334e-1_f64 * t11850 - t9214 + 0.19419375e1_f64 * t11853 + t12109 - 0.20128333333333333333e0_f64 * t11896 + 0.181155e1_f64 * t11899 + 0.12077e1_f64 * t11904 + 0.60385e0_f64 * t11908 - t12115 - 0.5519e-1_f64 * t11913 - 0.27595e-1_f64 * t11916 - 0.16557e0_f64 * t11919 + 0.33114e0_f64 * t11922 + 0.16557e0_f64 * t11925 + t12133;
    t12135
}
