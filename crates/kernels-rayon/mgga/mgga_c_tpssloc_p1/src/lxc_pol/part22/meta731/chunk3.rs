//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2400/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2400(t68457: f64, t68496: f64, t68532: f64, t68565: f64, t68594: f64, t68616: f64, t68637: f64, t68699: f64, t942: f64, t951: f64, t959: f64, t14473: f64, t5804: f64) -> (f64, f64, f64) {
    let t68702 = t68457 + t68496 + t68532 + t68565 + t68594 + t68616 + t68637 + t68699;
    let t68706 = 0.5848223622634646207e0_f64 * t959 * t942 * t68702 * t951;
    let t68708 = 0.35089341735807877242e1_f64 * t14473 * t5804;
    (t68702, t68706, t68708)
}
