//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1377/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1377<F: Float>(t111419: F, t113019: F, t113022: F, t113025: F, t113053: F, t113054: F, t116867: F, t116876: F, t1458: F, t1914: F, t1921: F, t2168: F, t2172: F, t25049: F, t25072: F, t3: F, t30975: F, t30993: F, t575: F, t6937: F, t6951: F, t8241: F, t8249: F) -> F {
    let tv4rho3sigma11 = t116867 * t3 * t575 + t116876 * t1458 + F::cast_from(3.0_f64) * t1914 * t30993 + F::cast_from(3.0_f64) * t1921 * t30975 + t2168 * t25072 + t2172 * t25049 + F::cast_from(3.0_f64) * t6937 * t8249 + F::cast_from(3.0_f64) * t6951 * t8241 + F::cast_from(3.0_f64) * t111419 + F::cast_from(3.0_f64) * t113019 + F::cast_from(6.0_f64) * t113022 + F::cast_from(3.0_f64) * t113025 + F::cast_from(3.0_f64) * t113053 + F::cast_from(6.0_f64) * t113054;
    tv4rho3sigma11
}
