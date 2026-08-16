//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 279/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk279(t537: f64, t809: f64, t312: f64, t50: f64, t90: f64, t814: f64, t547: f64, t820: f64, t316: f64, t101: f64, t309: f64, t317: f64, t538: f64, t544: f64, t832: f64, t87: f64, t98: f64) -> (f64, f64, f64) {
    let t1569 = t809 * t537;
    let t1570 = t1569 * t312;
    let t1573 = t90 * t50;
    let t1574 = t1573 * t814;
    let t1579 = t820 * t547;
    let t1580 = t1579 * t316;
    let t1583 = t101 * t50;
    let t1584 = t1583 * t814;
    let t1587 = -50.0_f64 / 9.0_f64 * t309 * t538 + 20.0_f64 / 9.0_f64 * t87 * t1570 + 10.0_f64 / 3.0_f64 * t87 * t1574 - 50.0_f64 / 9.0_f64 * t544 * t317 + 20.0_f64 / 9.0_f64 * t98 * t1580 - 10.0_f64 / 3.0_f64 * t98 * t1584 - t832;
    (t1580, t1584, t1587)
}
