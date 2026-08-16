//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 587/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk587(t2732: f64, t829: f64, t2679: f64, t860: f64, t2684: f64, t235: f64, t2710: f64, t226: f64, t255: f64, t2613: f64, t2617: f64, t2729: f64, t808: f64, t812: f64, t861: f64, t863: f64) -> (f64, f64, f64, f64, f64) {
    let t2733 = t2732 * t829;
    let t2736 = t860 * t2679;
    let t2738 = t860 * t2684;
    let t2740 = t235 * t2710;
    let t2742 = t226 * t2740 + t255 * t2613 - 2.0_f64 * t2617 * t861 + 2.0_f64 * t2729 * t812 - 2.0_f64 * t2733 * t812 - t2736 * t812 - t2738 * t812 + 2.0_f64 * t808 * t863;
    (t2733, t2736, t2738, t2740, t2742)
}
