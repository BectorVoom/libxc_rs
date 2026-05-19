//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1406/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1406<F: Float>(t17331: F, t17335: F, t17337: F, t17339: F, t17342: F, t17344: F, t17347: F, t17350: F, t17353: F, t17355: F, t17358: F, t17360: F, t17362: F, t17364: F, t17366: F, t17368: F, t17371: F, t17374: F) -> F {
    let t18292 = -F::cast_from(0.101171875e-1_f64) * t17331 - F::cast_from(0.53958333333333333333e-1_f64) * t17335 + F::new(0.625e-1) * t17337 - F::cast_from(0.33333333333333333334e0_f64) * t17339 - F::cast_from(0.44965277777777777777e-2_f64) * t17342 - F::new(0.1875e0) * t17344 - F::cast_from(0.41666666666666666666e-1_f64) * t17347 + F::cast_from(0.44965277777777777777e-2_f64) * t17350 + F::cast_from(0.10791666666666666667e0_f64) * t17353 - F::new(0.9375e-1) * t17355 + F::new(0.375e0) * t17358 - F::new(0.9375e-1) * t17360 + F::cast_from(0.53958333333333333333e-1_f64) * t17362 - F::new(0.125e0) * t17364 + F::cast_from(0.26979166666666666666e-1_f64) * t17366 - F::cast_from(0.44965277777777777777e-2_f64) * t17368 + F::cast_from(0.20833333333333333333e-1_f64) * t17371 - F::new(0.4046875e-1) * t17374;
    t18292
}
