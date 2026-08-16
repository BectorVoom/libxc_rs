//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 902/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk902(t225: f64, t7319: f64, t23598: f64, t50: f64, t131: f64, t467: f64, t3030: f64, t461: f64, t1011: f64, t3508: f64, t1193: f64, t7372: f64) -> (f64, f64, f64, f64, f64) {
    let t24788 = t7319 * t225;
    let t24810 = t50 * t23598;
    let t24811 = t24810 * t131;
    let t24812 = t24811 * t467;
    let t24813 = t461 * t3030;
    let t24815 = t1011 * t3508;
    let t24826 = t7372 * t1193;
    (t24788, t24812, t24813, t24815, t24826)
}
