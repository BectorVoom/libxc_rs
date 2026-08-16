//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2998/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2998(t19968: f64, t4817: f64, t20054: f64, t4834: f64, t11631: f64, t11661: f64, t11859: f64, t15906: f64, t16081: f64, t19450: f64, t19572: f64, t19861: f64, t23485: f64, t23929: f64, t3091: f64, t3092: f64, t3117: f64, t4786: f64, t4866: f64, t54118: f64, t54123: f64, t54127: f64, t54696: f64, t55141: f64, t6339: f64, t66355: f64, t66362: f64, t66376: f64, t66403: f64, t66406: f64, t66423: f64, t66467: f64, t66470: f64, t78496: f64, t999: f64) -> f64 {
    let t79546 = t19968 * t4817;
    let t79548 = t4834 * t20054;
    let t79550 = -0.42874018118069736972e-3_f64 * t66355 - 0.42874018118069736972e-3_f64 * t66362 - 0.85748036236139473947e-3_f64 * t15906 * t3092 * t78496 * t11661 + 0.85748036236139473944e-3_f64 * t3091 * t3092 * t23485 * t4786 - 0.12862205435420921092e-2_f64 * t11859 * t3117 * t19572 * t23929 * t999 + 5.0_f64 / 1296.0_f64 * t54118 - 0.85748036236139473947e-3_f64 * t55141 * t19861 + t54123 - t54127 + 0.57165357490759649296e-3_f64 * t66376 + t66403 / 108.0_f64 + 7.0_f64 / 648.0_f64 * t66406 - t66423 / 144.0_f64 + 0.12862205435420921092e-2_f64 * t54696 * t6339 + 0.11433071498151929859e-2_f64 * t66467 + 0.38586616306262763276e-2_f64 * t16081 * t3117 * t19450 * t11631 * t4866 + 0.11433071498151929859e-2_f64 * t66470 + 0.28582678745379824648e-3_f64 * t79546 + 0.28582678745379824648e-3_f64 * t79548;
    t79550
}
