//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1339/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1339<F: Float>(t10825: F, t10826: F, t16902: F, t176: F, t2722: F, t275: F, t3608: F, t364: F, t40120: F, t40188: F, t4038: F, t4039: F, t4044: F, t4054: F, t49773: F, t56945: F, t56948: F, t56950: F, t56952: F, t57628: F, t57857: F, t58173: F, sigma0: F) -> F {
    let t58180 = F::cast_from(56.0_f64) / F::cast_from(27.0_f64) * t4054 * t16902 + F::cast_from(28.0_f64) / F::cast_from(9.0_f64) * t4038 * t10825 * t10826 * t57628 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t4038 * t2722 * t4039 * t57857 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t4038 * t3608 * t4044 * t57857 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t40120 - t56945 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t40188 + t176 * t58173 * t275 * sigma0 * t364 / F::cast_from(2.0_f64) - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t49773 - t56948 + t56950 + t56952;
    t58180
}
