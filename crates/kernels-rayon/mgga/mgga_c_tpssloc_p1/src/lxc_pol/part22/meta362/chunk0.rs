//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1604/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1604(t17297: f64, t942: f64, t951: f64, t959: f64, t2940: f64, t5812: f64, t5811: f64, t952: f64, t10296: f64, t10556: f64, t10784: f64, t10785: f64, t13552: f64, t13566: f64, t14287: f64, t14291: f64, t17173: f64, t17180: f64, t17185: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17299 = t942 * t17297 * t951;
    let t17301 = 0.5848223622634646207e0_f64 * t959 * t17299;
    let t17303 = 0.17315859105681463759e2_f64 * t2940 * t5812;
    let t17304 = t5811 * t952;
    let t17306 = 0.35089341735807877242e1_f64 * t959 * t17304;
    let t17325 = 0.20659e1_f64 * t17173 - t14287 + 0.4630888888888888889e-1_f64 * t13552 + t14291 - 0.68863333333333333332e0_f64 * t13566 - 0.11577222222222222222e0_f64 * t10296 - t10784 - t10785 - 0.34431666666666666667e0_f64 * t17180 + 0.103295e1_f64 * t17185 - 0.22954444444444444444e0_f64 * t10556;
    (t17299, t17301, t17303, t17304, t17306, t17325)
}
