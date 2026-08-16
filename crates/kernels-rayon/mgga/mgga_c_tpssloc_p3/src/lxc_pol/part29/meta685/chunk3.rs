//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2341/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2341(t2240: f64, t27363: f64, t33: f64, t24520: f64, t24526: f64, t26063: f64, t26067: f64, t27308: f64, t27311: f64, t27365: f64, t6492: f64, t6495: f64, t7246: f64, t90177: f64, t90227: f64, t90232: f64, t90334: f64) -> f64 {
    let t96072 = t2240 * t33 * t27363;
    let t96077 = 2.0_f64 / 3.0_f64 * t6495 * t27308 + 5.0_f64 / 3.0_f64 * t7246 * t90177 + 2.0_f64 / 3.0_f64 * t6495 * t27311 + 5.0_f64 / 3.0_f64 * t24520 * t26063 + 5.0_f64 / 3.0_f64 * t24526 * t26063 + 5.0_f64 / 6.0_f64 * t7246 * t90227 + 5.0_f64 / 3.0_f64 * t7246 * t90232 + 5.0_f64 / 3.0_f64 * t24520 * t26067 + 5.0_f64 / 3.0_f64 * t24526 * t26067 + 5.0_f64 / 6.0_f64 * t7246 * t90334 + 5.0_f64 / 3.0_f64 * t96072 * t6492 + 2.0_f64 / 3.0_f64 * t6495 * t27365;
    t96077
}
