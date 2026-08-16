//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1599/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1599(t1156: f64, t18785: f64, t11297: f64, t1148: f64, t18676: f64, t18679: f64, t18682: f64, t18685: f64, t18688: f64, t18690: f64, t18692: f64, t18694: f64, t18696: f64, t18711: f64, t3371: f64, t6069: f64, t6085: f64) -> f64 {
    let t18786 = t18785 * t1156;
    let t18789 = -t18676 - t18679 + t18682 + t18685 - t18688 - t18690 - t18692 + t18694 - t18696 - 0.19751673498613801407e-1_f64 * t18711 - 0.11696447245269292414e1_f64 * t11297 * t6069 + 0.5848223622634646207e0_f64 * t3371 * t6085 + 0.5848223622634646207e0_f64 * t1148 * t18786;
    t18789
}
