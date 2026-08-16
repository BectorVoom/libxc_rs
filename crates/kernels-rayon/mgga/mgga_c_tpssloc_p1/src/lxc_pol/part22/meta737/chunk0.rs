//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2420/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2420(t49486: f64, t5695: f64, t10655: f64, t21253: f64, t17521: f64, t48763: f64, t21347: f64, t300: f64, t961: f64, t10702: f64, t14395: f64, t5726: f64, t912: f64) -> (f64, f64, f64, f64, f64) {
    let t69003 = 6.0_f64 * t49486 * t5695;
    let t69005 = 6.0_f64 * t10655 * t21253;
    let t69011 = 0.2894756309764656312e3_f64 * t48763 * t17521;
    let t69012 = t300 * t21347;
    let t69014 = 0.5848223622634646207e0_f64 * t69012 * t961;
    let t69018 = 0.1551780387578202009e4_f64 * t10702 * t5726 * t14395 * t912;
    (t69003, t69005, t69011, t69014, t69018)
}
