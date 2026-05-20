//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2998/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2998<F: Float>(t19968: F, t4817: F, t20054: F, t4834: F, t11631: F, t11661: F, t11859: F, t15906: F, t16081: F, t19450: F, t19572: F, t19861: F, t23485: F, t23929: F, t3091: F, t3092: F, t3117: F, t4786: F, t4866: F, t54118: F, t54123: F, t54127: F, t54696: F, t55141: F, t6339: F, t66355: F, t66362: F, t66376: F, t66403: F, t66406: F, t66423: F, t66467: F, t66470: F, t78496: F, t999: F) -> F {
    let t79546 = t19968 * t4817;
    let t79548 = t4834 * t20054;
    let t79550 = -F::cast_from(0.42874018118069736972e-3_f64) * t66355 - F::cast_from(0.42874018118069736972e-3_f64) * t66362 - F::cast_from(0.85748036236139473947e-3_f64) * t15906 * t3092 * t78496 * t11661 + F::cast_from(0.85748036236139473944e-3_f64) * t3091 * t3092 * t23485 * t4786 - F::cast_from(0.12862205435420921092e-2_f64) * t11859 * t3117 * t19572 * t23929 * t999 + F::new(5.0) / F::new(1296.0) * t54118 - F::cast_from(0.85748036236139473947e-3_f64) * t55141 * t19861 + t54123 - t54127 + F::cast_from(0.57165357490759649296e-3_f64) * t66376 + t66403 / F::new(108.0) + F::new(7.0) / F::new(648.0) * t66406 - t66423 / F::new(144.0) + F::cast_from(0.12862205435420921092e-2_f64) * t54696 * t6339 + F::cast_from(0.11433071498151929859e-2_f64) * t66467 + F::cast_from(0.38586616306262763276e-2_f64) * t16081 * t3117 * t19450 * t11631 * t4866 + F::cast_from(0.11433071498151929859e-2_f64) * t66470 + F::cast_from(0.28582678745379824648e-3_f64) * t79546 + F::cast_from(0.28582678745379824648e-3_f64) * t79548;
    t79550
}
