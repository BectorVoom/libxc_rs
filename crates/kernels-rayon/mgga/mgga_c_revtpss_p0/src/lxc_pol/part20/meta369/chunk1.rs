//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1345/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1345(t40165: f64, t760: f64, t39875: f64, t39960: f64, t39963: f64, t2523: f64, t9372: f64, t10600: f64, t14325: f64, t2258: f64, t4401: f64, t606: f64, t749: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40167 = 0.12304822629859687989e5_f64 * t760 * t40165;
    let t40169 = t39960 * t39875 * t39963;
    let t40171 = 0.91082604192152556044e5_f64 * t760 * t40169;
    let t40172 = t2523 * t9372;
    let t40173 = 0.4101607543286562663e4_f64 * t40172;
    let t40175 = 144.0_f64 * t14325 * t10600;
    let t40178 = t4401 * t749 * t606 * t2258;
    (t40167, t40169, t40171, t40173, t40175, t40178)
}
