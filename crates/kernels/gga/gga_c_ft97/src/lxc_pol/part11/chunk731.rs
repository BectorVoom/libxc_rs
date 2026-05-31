//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 731/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk731<F: Float>(t2493: F, t9757: F, t462: F, t9897: F, t9900: F, t9903: F, t9905: F, t9907: F, t9910: F, t9913: F, t9917: F, t9922: F, t9925: F) -> (F, F) {
    let t9928 = t2493 * t9757;
    let t9930 = -F::cast_from(2.0_f64) * t462 * t9897 - F::cast_from(2.0_f64) * t462 * t9900 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t9903 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t9905 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t9907 - F::cast_from(2.0_f64) * t462 * t9910 + F::cast_from(2.0_f64) * t462 * t9913 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t462 * t9917 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t462 * t9922 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t462 * t9925 + t462 * t9928;
    (t9928, t9930)
}
