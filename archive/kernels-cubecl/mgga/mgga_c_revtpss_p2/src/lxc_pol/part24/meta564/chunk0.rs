//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1702/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1702<F: Float>(t1011: F, t1042: F, t15935: F, t16095: F, t23857: F, t23886: F, t3092: F, t3117: F, t3127: F, t3162: F, t43044: F, t4578: F, t4834: F, t4919: F, t53762: F, t65859: F, t66022: F, t66029: F, t66141: F, t66218: F, t79290: F, t79309: F, t79315: F, t88124: F, t88925: F, t89084: F) -> F {
    let t89121 = -t1011 * t4919 * t88124 / F::cast_from(6.0_f64) - F::cast_from(0.3811023832717309953e-3_f64) * t65859 - F::cast_from(0.28582678745379824648e-3_f64) * t66022 - F::cast_from(0.57165357490759649296e-3_f64) * t66029 - F::cast_from(0.17149607247227894789e-2_f64) * t79290 - F::cast_from(0.34299214494455789577e-2_f64) * t3127 * t1042 * t15935 * t88925 - F::cast_from(0.57165357490759649296e-2_f64) * t4834 * t23886 + F::cast_from(0.34299214494455789578e-2_f64) * t79309 + F::cast_from(0.16937883700965822013e-3_f64) * t53762 + t79315 / F::cast_from(36.0_f64) + F::cast_from(0.34299214494455789577e-2_f64) * t16095 * t3092 * t4578 * t23857 - F::cast_from(0.25724410870841842184e-2_f64) * t43044 * t3117 * t89084 * t3162 - F::cast_from(0.28582678745379824648e-3_f64) * t66141 - t66218 / F::cast_from(162.0_f64);
    t89121
}
