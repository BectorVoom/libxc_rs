//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 981/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk981<F: Float>(t3547: F, t986: F, t1422: F, t2554: F, t2539: F, t3550: F, t2562: F, t3546: F, t1421: F, t7061: F, t2538: F, t2560: F, t3551: F, t3565: F, t3584: F, t7002: F, t7059: F, t7104: F, t7133: F, t7159: F, t8977: F, t8979: F, t8982: F, t8985: F) -> (F, F, F, F, F, F, F) {
    let t9073 = t3547 * t986;
    let t9076 = t1422 * t2554;
    let t9079 = t3550 * t2539;
    let t9082 = t3546 * t2562;
    let t9083 = t9082 * t986;
    let t9086 = t3550 * t2554;
    let t9089 = t1421 * t7061;
    let t9090 = t9089 * t2539;
    let t9097 = F::cast_from(0.64327917994770140268e2_f64) * t7159 * t3551 - F::cast_from(4.0_f64) * t2538 * t9073 - F::cast_from(2.0_f64) * t2538 * t9076 - F::cast_from(0.19298375398431042081e3_f64) * t7002 * t9079 + F::cast_from(0.64327917994770140268e2_f64) * t2560 * t9083 + F::cast_from(0.32163958997385070134e2_f64) * t2560 * t9086 + F::cast_from(0.2069040516770936012e4_f64) * t7059 * t9090 - F::cast_from(0.23392894490538584828e1_f64) * t7133 * t3565 + F::cast_from(0.34631718211362927518e2_f64) * t7104 * t3584 + t8977 - t8979 + t8982 + t8985;
    (t9073, t9076, t9079, t9083, t9086, t9090, t9097)
}
