//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1035/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1035<F: Float>(t40590: F, t8793: F, t115530: F, t117284: F, t122227: F, t122235: F, t122247: F, t1375: F, t16030: F, t16439: F, t2091: F, t2092: F, t26224: F, t27114: F, t32161: F, t33798: F, t33804: F, t33810: F, t3758: F, t3887: F, t5215: F, t5321: F, t5325: F, t8801: F, t93316: F) -> F {
    let t124103 = t40590 * t8793;
    let t124122 = -F::cast_from(6.0_f64) * t3758 * t33810 - F::cast_from(0.19739208802178717238e0_f64) * t122227 - t16030 * t8801 + F::cast_from(4.0_f64) * t3758 * t33804 - F::cast_from(0.3289868133696452873e-1_f64) * t122235 + F::cast_from(24.0_f64) * t26224 * t124103 * t5325 + F::cast_from(4.0_f64) * t1375 * t3887 * t2091 * t27114 - F::cast_from(2.0_f64) * t93316 * t2092 - F::cast_from(0.76763589786250567037e-1_f64) * t115530 - F::cast_from(6.0_f64) * t5321 * t32161 - t16439 * t8801 - t117284 + F::cast_from(0.16449340668482264365e-1_f64) * t122247 + F::cast_from(2.0_f64) * t3758 * t33798 - F::cast_from(6.0_f64) * t5215 * t32161;
    t124122
}
