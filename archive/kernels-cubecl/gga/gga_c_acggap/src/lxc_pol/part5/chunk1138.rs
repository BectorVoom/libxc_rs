//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1138/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1138<F: Float>(t3382: F, t5986: F, t3409: F, t5801: F, t12738: F, t6184: F, t1165: F, t1173: F, t12498: F, t12511: F, t12516: F, t12529: F, t12532: F, t1889: F, t20400: F, t3176: F, t3196: F, t3403: F, t4289: F, t5720: F, t5862: F) -> F {
    let t20405 = t3382 * t5986;
    let t20407 = t3409 * t5801;
    let t20409 = t12738 * t6184;
    let t20416 = -F::cast_from(0.85748036236139473944e-2_f64) * t3403 * t1165 * t1889 * t3196 + F::cast_from(0.17149607247227894789e-2_f64) * t1173 * t1165 * t5862 * t3196 + F::cast_from(0.34299214494455789578e-2_f64) * t1173 * t1165 * t4289 * t5720 - F::cast_from(0.34299214494455789578e-2_f64) * t1173 * t1165 * t20400 * t3176 - F::cast_from(0.51448821741683684366e-2_f64) * t20405 + F::cast_from(0.80031500487063509014e-2_f64) * t20407 + F::cast_from(0.40015750243531754508e-2_f64) * t20409 - F::cast_from(0.24009450146119052705e-1_f64) * t12498 + F::cast_from(0.24009450146119052705e-1_f64) * t12511 + F::cast_from(0.85748036236139473944e-3_f64) * t12516 + F::cast_from(0.85748036236139473944e-3_f64) * t12529 - F::cast_from(0.34013387707001991332e-1_f64) * t12532;
    t20416
}
