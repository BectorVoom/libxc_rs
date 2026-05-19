//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1285/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1285<F: Float>(t13298: F, t13299: F, t1849: F, t4210: F, t12572: F, t6376: F, t1137: F, t6297: F, t4389: F, t5755: F, t1165: F, t1173: F, t13591: F, t14221: F, t14228: F, t14233: F, t14239: F, t14242: F, t18426: F, t301: F, t5275: F, t5852: F, t5853: F, t5867: F) -> F {
    let t23781 = t13298 * t13299 * t1849 * t4210;
    let t23787 = t12572 * t6376;
    let t23789 = t1137 * t6297;
    let t23792 = t4389 * t5755;
    let t23803 = F::cast_from(0.68598428988911579156e-2_f64) * t23781 - F::cast_from(0.16006300097412701803e-1_f64) * t14221 - F::cast_from(0.42874018118069736972e-3_f64) * t14228 - F::cast_from(0.42874018118069736972e-3_f64) * t14233 + F::cast_from(0.42874018118069736972e-3_f64) * t14239 + t14242 + F::new(7.0) / F::new(6.0) * t23787 - F::new(7.0) / F::new(36.0) * t23789 + F::cast_from(0.16006300097412701803e-1_f64) * t18426 - F::cast_from(0.16006300097412701803e-1_f64) * t23792 - F::cast_from(0.34299214494455789578e-2_f64) * t1173 * t1165 * t5867 * t5275 + F::cast_from(0.10289764348336736874e-1_f64) * t13591 * t1165 * t5852 * t5853 * t301;
    t23803
}
