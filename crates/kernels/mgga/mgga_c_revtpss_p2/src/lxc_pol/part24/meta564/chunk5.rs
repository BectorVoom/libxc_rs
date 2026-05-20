//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1707/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1707<F: Float>(t1011: F, t1012: F, t1015: F, t15707: F, t1675: F, t19968: F, t23485: F, t23859: F, t23911: F, t23976: F, t23980: F, t3091: F, t3092: F, t42518: F, t43223: F, t4834: F, t54687: F, t6323: F, t6327: F, t79559: F, t79742: F, t79744: F, t79758: F, t87126: F, t87145: F) -> F {
    let t89283 = F::cast_from(0.34299214494455789577e-2_f64) * t3091 * t3092 * t23485 * t23911 + F::cast_from(0.57165357490759649296e-3_f64) * t79559 * t1675 + F::cast_from(0.85748036236139473944e-3_f64) * t19968 * t6323 + F::cast_from(0.14291339372689912324e-2_f64) * t19968 * t6327 + F::cast_from(0.57165357490759649296e-3_f64) * t4834 * t23976 + F::cast_from(0.2540682555144873302e-2_f64) * t4834 * t23980 - F::cast_from(0.17149607247227894789e-2_f64) * t15707 * t23859 - F::cast_from(0.2540682555144873302e-3_f64) * t54687 + t1011 * t1012 * t42518 * t87145 / F::new(6.0) + t1011 * t1012 * t1015 * t87126 / F::new(288.0) + F::new(35.0) / F::new(972.0) * t1011 * t1012 * t43223 * t87145 - F::cast_from(0.34299214494455789578e-2_f64) * t79742 + F::cast_from(0.34299214494455789578e-2_f64) * t79744 - F::cast_from(0.57165357490759649296e-3_f64) * t79758;
    t89283
}
