//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1174/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1174<F: Float>(t1165: F, t1180: F, t1181: F, t13656: F, t1439: F, t15995: F, t16407: F, t16409: F, t16415: F, t16417: F, t16421: F, t16423: F, t1866: F, t1899: F, t335: F, t3396: F, t4533: F, t4643: F, t4680: F, t4752: F, t4838: F, t530: F, t5902: F, t930: F, t960: F) -> F {
    let t21290 = t335 * t960 * t530 * t4838 / F::cast_from(24.0_f64) + t335 * t13656 * t1866 / F::cast_from(24.0_f64) + F::cast_from(0.13719685797782315831e-1_f64) * t3396 * t1181 * t15995 * t1439 + F::cast_from(0.13719685797782315831e-1_f64) * t3396 * t1181 * t4643 * t4752 + F::cast_from(0.68598428988911579156e-2_f64) * t3396 * t1181 * t4643 * t4533 + F::cast_from(0.34299214494455789578e-2_f64) * t16407 + F::cast_from(0.42874018118069736972e-3_f64) * t1180 * t1165 * t1899 * t930 + F::cast_from(0.13719685797782315831e-1_f64) * t3396 * t4680 * t5902 - F::cast_from(0.64025200389650807212e-1_f64) * t16409 - F::cast_from(0.17149607247227894789e-1_f64) * t16415 - F::cast_from(0.64025200389650807212e-1_f64) * t16417 + F::cast_from(0.13719685797782315831e-1_f64) * t16421 + F::cast_from(0.64025200389650807212e-1_f64) * t16423;
    t21290
}
