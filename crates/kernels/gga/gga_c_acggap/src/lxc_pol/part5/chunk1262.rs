//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1262/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1262<F: Float>(t3409: F, t5873: F, t1106: F, t1181: F, t1884: F, t4282: F, t12457: F, t1817: F, t1089: F, t1095: F, t1173: F, t13889: F, t13929: F, t13934: F, t13936: F, t13939: F, t13943: F, t1749: F, t17895: F, t17902: F, t372: F, t418: F, t5506: F) -> F {
    let t23274 = t3409 * t5873;
    let t23285 = t4282 * t1181 * t1884 * t1106;
    let t23288 = t12457 * t1817;
    let t23295 = F::cast_from(0.20007875121765877254e-2_f64) * t23274 - F::new(35.0) / F::new(108.0) * t17895 + F::cast_from(0.80031500487063509016e-2_f64) * t13929 - F::cast_from(0.40015750243531754508e-2_f64) * t13934 + F::cast_from(0.40015750243531754508e-2_f64) * t13936 - t13939 + t13943 + F::cast_from(0.34299214494455789578e-2_f64) * t1173 * t13889 * t1749 - F::cast_from(0.17149607247227894789e-1_f64) * t23285 + F::new(455.0) / F::new(324.0) * t17902 + F::cast_from(0.11337795902333997111e-1_f64) * t23288 + F::cast_from(0.34299214494455789578e-2_f64) * t418 * t1089 * t1095 * t5506 * t372;
    t23295
}
