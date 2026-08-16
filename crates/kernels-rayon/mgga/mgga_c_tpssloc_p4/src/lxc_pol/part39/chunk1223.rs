//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1223/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1223(t3536: f64, t4997: f64, t248: f64, t3570: f64, t5012: f64, t1213: f64, t3535: f64, t5018: f64, t1202: f64, t5023: f64, t1742: f64, t3036: f64) -> (f64, f64, f64, f64, f64) {
    let t15490 = t3536 * t4997 / 2304.0_f64;
    let t15492 = t248 * t3570 * t5012;
    let t15494 = t1213 * t15492 / 2304.0_f64;
    let t15495 = t3535 * t5018;
    let t15498 = t1202 * t5023;
    let t15501 = t1742 * t3036;
    (t15490, t15494, t15495, t15498, t15501)
}
