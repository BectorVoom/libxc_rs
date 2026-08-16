//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1339/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1339(t10825: f64, t10826: f64, t16902: f64, t176: f64, t2722: f64, t275: f64, t3608: f64, t364: f64, t40120: f64, t40188: f64, t4038: f64, t4039: f64, t4044: f64, t4054: f64, t49773: f64, t56945: f64, t56948: f64, t56950: f64, t56952: f64, t57628: f64, t57857: f64, t58173: f64, sigma0: f64) -> f64 {
    let t58180 = 56.0_f64 / 27.0_f64 * t4054 * t16902 + 28.0_f64 / 9.0_f64 * t4038 * t10825 * t10826 * t57628 - 4.0_f64 / 3.0_f64 * t4038 * t2722 * t4039 * t57857 + 8.0_f64 / 9.0_f64 * t4038 * t3608 * t4044 * t57857 - 2.0_f64 / 9.0_f64 * t40120 - t56945 - 2.0_f64 / 3.0_f64 * t40188 + t176 * t58173 * t275 * sigma0 * t364 / 2.0_f64 - 4.0_f64 / 3.0_f64 * t49773 - t56948 + t56950 + t56952;
    t58180
}
