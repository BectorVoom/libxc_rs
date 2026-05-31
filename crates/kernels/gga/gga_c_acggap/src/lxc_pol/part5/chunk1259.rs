//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1259/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1259<F: Float>(t1881: F, t3670: F, t1896: F, t3237: F, t1181: F, t13911: F, t13915: F, t13919: F, t13923: F, t13927: F, t17798: F, t17804: F, t17811: F, t4665: F, t4680: F, t4735: F, t6337: F, t6338: F) -> F {
    let t23207 = t3670 * t1881;
    let t23209 = t3237 * t1896;
    let t23226 = F::cast_from(0.34013387707001991332e-1_f64) * t23207 - F::cast_from(0.40015750243531754508e-2_f64) * t23209 + F::cast_from(0.13719685797782315831e-1_f64) * t17798 + F::cast_from(0.51448821741683684367e-2_f64) * t4735 * t1181 * t6337 * t4665 + F::cast_from(0.68598428988911579156e-2_f64) * t17804 - F::cast_from(0.42874018118069736972e-3_f64) * t13911 + F::cast_from(0.10289764348336736873e-1_f64) * t4735 * t4680 * t6338 + F::cast_from(0.17149607247227894789e-2_f64) * t13915 - F::cast_from(0.51448821741683684367e-2_f64) * t13919 - F::cast_from(0.34299214494455789578e-2_f64) * t13923 - F::cast_from(0.68598428988911579156e-2_f64) * t13927 + F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t17811;
    t23226
}
