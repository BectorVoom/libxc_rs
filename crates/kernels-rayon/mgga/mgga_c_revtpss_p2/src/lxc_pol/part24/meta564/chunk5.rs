//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1707/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1707(t1011: f64, t1012: f64, t1015: f64, t15707: f64, t1675: f64, t19968: f64, t23485: f64, t23859: f64, t23911: f64, t23976: f64, t23980: f64, t3091: f64, t3092: f64, t42518: f64, t43223: f64, t4834: f64, t54687: f64, t6323: f64, t6327: f64, t79559: f64, t79742: f64, t79744: f64, t79758: f64, t87126: f64, t87145: f64) -> f64 {
    let t89283 = 0.34299214494455789577e-2_f64 * t3091 * t3092 * t23485 * t23911 + 0.57165357490759649296e-3_f64 * t79559 * t1675 + 0.85748036236139473944e-3_f64 * t19968 * t6323 + 0.14291339372689912324e-2_f64 * t19968 * t6327 + 0.57165357490759649296e-3_f64 * t4834 * t23976 + 0.2540682555144873302e-2_f64 * t4834 * t23980 - 0.17149607247227894789e-2_f64 * t15707 * t23859 - 0.2540682555144873302e-3_f64 * t54687 + t1011 * t1012 * t42518 * t87145 / 6.0_f64 + t1011 * t1012 * t1015 * t87126 / 288.0_f64 + 35.0_f64 / 972.0_f64 * t1011 * t1012 * t43223 * t87145 - 0.34299214494455789578e-2_f64 * t79742 + 0.34299214494455789578e-2_f64 * t79744 - 0.57165357490759649296e-3_f64 * t79758;
    t89283
}
