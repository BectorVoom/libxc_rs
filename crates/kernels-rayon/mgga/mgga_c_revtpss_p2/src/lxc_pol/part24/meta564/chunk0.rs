//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1702/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1702(t1011: f64, t1042: f64, t15935: f64, t16095: f64, t23857: f64, t23886: f64, t3092: f64, t3117: f64, t3127: f64, t3162: f64, t43044: f64, t4578: f64, t4834: f64, t4919: f64, t53762: f64, t65859: f64, t66022: f64, t66029: f64, t66141: f64, t66218: f64, t79290: f64, t79309: f64, t79315: f64, t88124: f64, t88925: f64, t89084: f64) -> f64 {
    let t89121 = -t1011 * t4919 * t88124 / 6.0_f64 - 0.3811023832717309953e-3_f64 * t65859 - 0.28582678745379824648e-3_f64 * t66022 - 0.57165357490759649296e-3_f64 * t66029 - 0.17149607247227894789e-2_f64 * t79290 - 0.34299214494455789577e-2_f64 * t3127 * t1042 * t15935 * t88925 - 0.57165357490759649296e-2_f64 * t4834 * t23886 + 0.34299214494455789578e-2_f64 * t79309 + 0.16937883700965822013e-3_f64 * t53762 + t79315 / 36.0_f64 + 0.34299214494455789577e-2_f64 * t16095 * t3092 * t4578 * t23857 - 0.25724410870841842184e-2_f64 * t43044 * t3117 * t89084 * t3162 - 0.28582678745379824648e-3_f64 * t66141 - t66218 / 162.0_f64;
    t89121
}
