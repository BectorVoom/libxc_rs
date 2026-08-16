//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1141/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1141(t1061: f64, t5130: f64, t1531: f64, t4142: f64, t5149: f64, t5146: f64, t12083: f64, t12269: f64, t15612: f64, t15615: f64, t15618: f64, t15621: f64, t15625: f64, t15628: f64, t15632: f64, t2930: f64, t2955: f64, t4125: f64, t4147: f64, t9424: f64) -> f64 {
    let t15723 = t5130 * t1061;
    let t15726 = t1531 * t4142;
    let t15729 = t5149 * t1061;
    let t15732 = t5146 * t1061;
    let t15735 = -t15612 + t15615 + t15618 + t15621 - t15625 - t15628 - t15632 - 4.0_f64 * t12083 * t4125 + 0.64327917994770140268e2_f64 * t12269 * t4147 + 6.0_f64 * t2955 * t15723 - 4.0_f64 * t2930 * t15726 - 0.19298375398431042081e3_f64 * t9424 * t15729 - 2.0_f64 * t2930 * t15732;
    t15735
}
