//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 808/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk808<F: Float>(t3793: F, t3808: F, t3812: F, t5222: F, t5224: F, t5226: F, t5229: F, t5240: F, t5243: F, t5558: F, t5626: F, t5709: F, t5766: F, t5837: F, t5882: F, t5943: F, t5999: F, t6095: F, t6133: F, t6182: F, t6223: F, t6268: F, t6317: F, t6366: F, t6371: F, t6408: F) -> F {
    let t6413 = t5626 + t3808 + F::cast_from(0.85748036236139473944e-3_f64) * t3812 + t5558 + t5837 - F::cast_from(0.80031500487063509016e-2_f64) * t3793 - t5229 + t6095 + t6133 + t5243 - F::cast_from(0.85748036236139473944e-3_f64) * t5226 + t5709 + t5766 + t5943 + F::cast_from(0.85748036236139473944e-3_f64) * t5240 + t5882 - t5222 - t5224 + t5999 + t6408 + t6182 + t6223 + t6268 + t6317 + t6366 - F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t6371;
    t6413
}
