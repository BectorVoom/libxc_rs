//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 960/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk960(t25482: f64, t25527: f64, t25560: f64, t25729: f64, t1055: f64, t23384: f64, t7566: f64, t23394: f64, t4664: f64, t6704: f64, t1634: f64, t6815: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25731 = t25482 + t25527 + t25560 + t25729;
    let t25732 = t1055 * t25731;
    let t25736 = t23384 * t7566;
    let t25738 = t23394 * t4664;
    let t25739 = t6704 * t25738;
    let t25742 = t6815 * t1634;
    (t25731, t25732, t25736, t25738, t25739, t25742)
}
